/*
 * AST-based bundler core logic
 * Please don't touch I will nuke the repo if you do
 */

use crate::file_resolver::FileResolver;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::{File as SynFile, Item, ItemUse, UseTree, UsePath};
use quote::ToTokens;

pub struct AstBundler<'a> {
    resolver: &'a FileResolver,
    bundled_items: Vec<Item>,
    processed_modules: HashSet<String>,
    use_statements: HashSet<String>,
    cp_lib_aliases: HashSet<String>,
}

struct CpLibPathCollector {
    paths: Vec<String>,
}

impl CpLibPathCollector {
    fn new() -> Self {
        Self { paths: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for CpLibPathCollector {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        if let Some(first) = path.segments.first() {
            if first.ident == "cp_lib" {
                let joined = path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                self.paths.push(joined);
            }
        }
        syn::visit::visit_path(self, path);
    }
}

struct StripPaths<'a> {
    aliases: &'a HashSet<String>,
}

impl<'a> VisitMut for StripPaths<'a> {
    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        if let Some(first) = path.segments.first() {
            let first_ident = first.ident.to_string();
            let should_strip = first.ident == "cp_lib" || (path.segments.len() > 1 && self.aliases.contains(&first_ident));

            if should_strip {
                if let Some(last) = path.segments.last().cloned() {
                    path.leading_colon = None;
                    path.segments.clear();
                    path.segments.push(last);
                }
            }
        }

        syn::visit_mut::visit_path_mut(self, path);
    }
}

impl<'a> AstBundler<'a> {
    pub fn new(resolver: &'a FileResolver) -> Self {
        Self {
            resolver,
            bundled_items: Vec::new(),
            processed_modules: HashSet::new(),
            use_statements: HashSet::new(),
            cp_lib_aliases: HashSet::new(),
        }
    }

    fn inline_cp_lib_paths_in_item(&mut self, item: &Item) -> Result<(), Box<dyn std::error::Error>> {
        let mut collector = CpLibPathCollector::new();
        collector.visit_item(item);
        for path in collector.paths {
            if path.starts_with("cp_lib::") {
                self.resolve_and_inline_module(&path)?;
            }
        }
        Ok(())
    }

    fn rewrite_cp_lib_paths_in_item(&self, item: &mut Item) {
        let mut rewriter = StripPaths {
            aliases: &self.cp_lib_aliases,
        };
        rewriter.visit_item_mut(item);
    }

    pub fn process_file_ast(&mut self, ast: &SynFile, _file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for item in &ast.items {
            match item {
                Item::Use(use_item) => {
                    self.process_use_item(use_item)?;
                }
                Item::Const(const_item) => {
                    // Skip _PROBLEM constant as it's not needed in final bundle
                    if const_item.ident != "_PROBLEM" {
                        self.inline_cp_lib_paths_in_item(item)?;
                        let mut cloned = item.clone();
                        self.rewrite_cp_lib_paths_in_item(&mut cloned);
                        self.bundled_items.push(cloned);
                    }
                }
                _ => {
                    // Include all other items (functions, structs, etc.)
                    self.inline_cp_lib_paths_in_item(item)?;
                    let mut cloned = item.clone();
                    self.rewrite_cp_lib_paths_in_item(&mut cloned);
                    self.bundled_items.push(cloned);
                }
            }
        }
        
        Ok(())
    }

    fn process_use_item(&mut self, use_item: &ItemUse) -> Result<(), Box<dyn std::error::Error>> {
        let mut use_paths = Vec::new();
        Self::collect_use_paths(String::new(), &use_item.tree, &mut use_paths);

        for path in use_paths {
            if path.starts_with("cp_lib::") {
                if let Some(alias) = path.split("::").last() {
                    if alias != "*" {
                        // Heuristic: only treat lowercase-leading names as module aliases.
                        // This prevents rewriting paths like `Scanner::from_reader`.
                        if alias.chars().next().is_some_and(|c| c.is_ascii_lowercase()) {
                            self.cp_lib_aliases.insert(alias.to_string());
                        }
                    }
                }
                // This is a cp_lib import, resolve and inline it
                self.resolve_and_inline_module(&path)?;
            } else if path.starts_with("std::") {
                // Standard library import, keep it but avoid duplicates
                let use_stmt = use_item.to_token_stream().to_string();
                let normalized: String = use_stmt.chars().filter(|c| !c.is_whitespace()).collect();
                // Skip std::io imports as we add our own comprehensive ones
                if !normalized.contains("std::io") {
                    self.use_statements.insert(use_stmt);
                }
            }
            // Skip other external crate imports
        }
        
        Ok(())
    }

    fn collect_use_paths(prefix: String, tree: &UseTree, out: &mut Vec<String>) {
        match tree {
            UseTree::Path(UsePath { ident, tree, .. }) => {
                let next_prefix = if prefix.is_empty() {
                    ident.to_string()
                } else {
                    format!("{}::{}", prefix, ident)
                };
                Self::collect_use_paths(next_prefix, tree, out);
            }
            UseTree::Name(name) => {
                let full = if prefix.is_empty() {
                    name.ident.to_string()
                } else {
                    format!("{}::{}", prefix, name.ident)
                };
                out.push(full);
            }
            UseTree::Rename(rename) => {
                let full = if prefix.is_empty() {
                    rename.ident.to_string()
                } else {
                    format!("{}::{}", prefix, rename.ident)
                };
                out.push(full);
            }
            UseTree::Glob(_) => {
                let full = if prefix.is_empty() {
                    "*".to_string()
                } else {
                    format!("{}::*", prefix)
                };
                out.push(full);
            }
            UseTree::Group(group) => {
                for item in &group.items {
                    Self::collect_use_paths(prefix.clone(), item, out);
                }
            }
        }
    }

    fn resolve_and_inline_module(&mut self, module_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Convert cp_lib::io::scanner::Scanner to file path
        let mut parts: Vec<&str> = module_path.split("::").collect();
        
        if parts.len() < 2 || parts[0] != "cp_lib" {
            return Ok(());
        }

        // Handle glob imports like cp_lib::algorithms::* by treating it as the module itself.
        if parts.last().is_some_and(|p| *p == "*") {
            parts.pop();
        }

        if parts.len() < 2 {
            return Ok(());
        }

        // For cp_lib::io::scanner::Scanner, we want io/scanner.rs
        // For cp_lib::algorithms::exponential (module import), we want algorithms/exponential.rs
        // For cp_lib::algorithms::exponential::pow_mod (item import), we want algorithms/exponential.rs
        let mut file_patterns = Vec::new();

        // Try interpreting the full path after cp_lib as a module path.
        // This fixes cases like: use cp_lib::algorithms::exponential;
        let module_candidate = parts[1..].join("/");
        file_patterns.push(format!("{}.rs", module_candidate));
        file_patterns.push(format!("{}/mod.rs", module_candidate));

        // Also try interpreting the last segment as a symbol inside a module.
        // This handles: use cp_lib::algorithms::exponential::pow_mod;
        if parts.len() >= 3 {
            let symbol_module_candidate = parts[1..parts.len() - 1].join("/");
            file_patterns.push(format!("{}.rs", symbol_module_candidate));
            file_patterns.push(format!("{}/mod.rs", symbol_module_candidate));
        }

        for pattern in file_patterns {
            if let Some(resolved_path) = self.resolver.resolve_module_file(&PathBuf::from(&pattern)) {
                let module_key = resolved_path.to_string_lossy().to_string();
                
                if !self.processed_modules.contains(&module_key) {
                    self.processed_modules.insert(module_key.clone());
                    
                    // Read and parse the module file
                    let content = fs::read_to_string(&resolved_path)?;
                    let module_ast: SynFile = syn::parse_str(&content)?;
                    
                    // Process the module recursively
                    self.process_module_ast(&module_ast, &resolved_path)?;
                    
                    // Stop after first successful resolution
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    fn process_module_ast(&mut self, ast: &SynFile, _file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for item in &ast.items {
            match item {
                Item::Use(use_item) => {
                    // Preserve std imports from modules and inline further cp_lib references.
                    self.process_use_item(use_item)?;
                    continue;
                }
                Item::Struct(_) | Item::Impl(_) | Item::Fn(_) | Item::Trait(_) | Item::Enum(_) => {
                    // Include all public items from modules
                    self.inline_cp_lib_paths_in_item(item)?;
                    let mut cloned = item.clone();
                    self.rewrite_cp_lib_paths_in_item(&mut cloned);
                    self.bundled_items.push(cloned);
                }
                _ => {
                    // Include other items too
                    self.inline_cp_lib_paths_in_item(item)?;
                    let mut cloned = item.clone();
                    self.rewrite_cp_lib_paths_in_item(&mut cloned);
                    self.bundled_items.push(cloned);
                }
            }
        }
        
        Ok(())
    }

    pub fn generate_bundled_code(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = String::new();
        
        // Header
        output.push_str("// Code bundled for competitive programming\n");
        output.push_str("// Generated automatically using AST-based bundler\n");
        output.push_str(&format!("// Generated at: {}\n\n", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs()));

        // Standard library imports
        output.push_str("use std::io::{self, BufRead, Cursor};\n");
        output.push_str("use std::collections::*;\n");
        output.push_str("use std::fmt::Debug;\n\n");

        // Additional use statements found in the code
        for use_stmt in &self.use_statements {
            if !use_stmt.contains("cp_lib") {
                output.push_str(&use_stmt);
                output.push('\n');
            }
        }
        
        if !self.use_statements.is_empty() {
            output.push('\n');
        }

        // Bundled items (library code first, then main code)
        output.push_str("// ==================== Library Code ====================\n");
        
        let mut main_items = Vec::new();
        let mut lib_items = Vec::new();
        
        for item in &self.bundled_items {
            match item {
                Item::Fn(func) if func.sig.ident == "main" => {
                    main_items.push(item);
                }
                Item::Const(const_item) if const_item.ident == "_PROBLEM" => {
                    // Skip _PROBLEM constant
                    continue;
                }
                _ => {
                    lib_items.push(item);
                }
            }
        }

        // Output library items first
        for item in lib_items {
            output.push_str(&item.to_token_stream().to_string());
            output.push_str("\n\n");
        }

        output.push_str("// ==================== Main Code ====================\n");
        
        // Output main function and other main-file items
        for item in main_items {
            output.push_str(&item.to_token_stream().to_string());
            output.push_str("\n\n");
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn resolves_module_imports_like_cp_lib_algorithms_exponential() {
        let temp_dir = tempdir().unwrap();
        let lib_root = temp_dir.path().join("cp-lib").join("src");
        fs::create_dir_all(lib_root.join("algorithms")).unwrap();

        fs::write(lib_root.join("algorithms").join("mod.rs"), "pub mod exponential;\n").unwrap();
        fs::write(
            lib_root.join("algorithms").join("exponential.rs"),
            "pub fn expo() -> i32 { 7 }\n",
        )
        .unwrap();

        let resolver = FileResolver::new(&lib_root);
        let mut bundler = AstBundler::new(&resolver);

        let main_src = "use cp_lib::algorithms::exponential; fn main() { let _ = exponential::expo(); }";
        let main_ast: SynFile = syn::parse_str(main_src).unwrap();

        bundler.process_file_ast(&main_ast, Path::new("src/main.rs")).unwrap();

        let bundled = bundler.generate_bundled_code().unwrap();
        assert!(bundled.contains("fn expo"));
        assert!(!bundled.contains("cp_lib"));
        assert!(!bundled.contains("exponential :: expo"));
    }

    #[test]
    fn rewrites_fully_qualified_cp_lib_paths() {
        let temp_dir = tempdir().unwrap();
        let lib_root = temp_dir.path().join("cp-lib").join("src");
        fs::create_dir_all(lib_root.join("algorithms")).unwrap();

        fs::write(lib_root.join("algorithms").join("mod.rs"), "pub mod exponential;\n").unwrap();
        fs::write(
            lib_root.join("algorithms").join("exponential.rs"),
            "pub fn binpow(_a: i64, _b: i64) -> i64 { 1 }\n",
        )
        .unwrap();

        let resolver = FileResolver::new(&lib_root);
        let mut bundler = AstBundler::new(&resolver);

        let main_src = "fn main(){ let x = cp_lib::algorithms::exponential::binpow(2, 8); println!(\"{}\", x); }";
        let main_ast: SynFile = syn::parse_str(main_src).unwrap();

        bundler.process_file_ast(&main_ast, Path::new("src/main.rs")).unwrap();
        let bundled = bundler.generate_bundled_code().unwrap();

        assert!(bundled.contains("fn binpow"));
        assert!(!bundled.contains("cp_lib"));
    }
}