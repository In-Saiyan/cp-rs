/*
 * AST-based bundler core logic
 */

use crate::file_resolver::FileResolver;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{File as SynFile, Item, ItemUse, UseTree, UsePath};
use quote::ToTokens;

pub struct AstBundler<'a> {
    resolver: &'a FileResolver,
    bundled_items: Vec<Item>,
    processed_modules: HashSet<String>,
    use_statements: HashSet<String>,
}

impl<'a> AstBundler<'a> {
    pub fn new(resolver: &'a FileResolver) -> Self {
        Self {
            resolver,
            bundled_items: Vec::new(),
            processed_modules: HashSet::new(),
            use_statements: HashSet::new(),
        }
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
                        self.bundled_items.push(item.clone());
                    }
                }
                _ => {
                    // Include all other items (functions, structs, etc.)
                    self.bundled_items.push(item.clone());
                }
            }
        }
        
        Ok(())
    }

    fn process_use_item(&mut self, use_item: &ItemUse) -> Result<(), Box<dyn std::error::Error>> {
        let use_path = self.extract_use_path(&use_item.tree);
        
        if let Some(path) = use_path {
            if path.starts_with("cp_lib::") {
                // This is a cp_lib import, resolve and inline it
                self.resolve_and_inline_module(&path)?;
            } else if path.starts_with("std::") {
                // Standard library import, keep it but avoid duplicates
                let use_stmt = use_item.to_token_stream().to_string();
                // Skip std::io imports as we add our own comprehensive ones
                if !use_stmt.contains("std::io") {
                    self.use_statements.insert(use_stmt);
                }
            }
            // Skip other external crate imports
        }
        
        Ok(())
    }

    fn extract_use_path(&self, tree: &UseTree) -> Option<String> {
        match tree {
            UseTree::Path(UsePath { ident, tree, .. }) => {
                if let Some(rest) = self.extract_use_path(tree) {
                    Some(format!("{}::{}", ident, rest))
                } else {
                    Some(ident.to_string())
                }
            }
            UseTree::Name(name) => {
                Some(name.ident.to_string())
            }
            UseTree::Rename(rename) => {
                Some(rename.ident.to_string())
            }
            UseTree::Glob(_) => {
                Some("*".to_string())
            }
            UseTree::Group(_) => {
                // Handle grouped imports - for now, return None and handle individually
                None
            }
        }
    }

    fn resolve_and_inline_module(&mut self, module_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Convert cp_lib::io::scanner::Scanner to file path
        let parts: Vec<&str> = module_path.split("::").collect();
        
        if parts.len() < 2 || parts[0] != "cp_lib" {
            return Ok(());
        }

        println!("Resolving module: {}", module_path);

        // For cp_lib::io::scanner::Scanner, we want io/scanner.rs
        // For cp_lib::utils::math, we want utils/math.rs or utils/mod.rs
        let mut file_patterns = Vec::new();
        
        if parts.len() >= 3 {
            // cp_lib::io::scanner::Scanner -> try io/scanner.rs
            let path_parts = &parts[1..parts.len()-1]; // Skip cp_lib and Scanner
            file_patterns.push(format!("{}.rs", path_parts.join("/")));
            
            // Also try parent module
            if path_parts.len() > 1 {
                file_patterns.push(format!("{}/mod.rs", path_parts.join("/")));
            }
        } else if parts.len() == 2 {
            // cp_lib::something -> try something.rs or something/mod.rs
            file_patterns.push(format!("{}.rs", parts[1]));
            file_patterns.push(format!("{}/mod.rs", parts[1]));
        }

        for pattern in file_patterns {
            println!("  Trying pattern: {}", pattern);
            
            if let Some(resolved_path) = self.resolver.resolve_module_file(&PathBuf::from(&pattern)) {
                let module_key = resolved_path.to_string_lossy().to_string();
                
                if !self.processed_modules.contains(&module_key) {
                    self.processed_modules.insert(module_key.clone());
                    
                    println!("  Found and processing: {}", resolved_path.display());
                    
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

        println!("  Could not resolve module: {}", module_path);
        Ok(())
    }

    fn process_module_ast(&mut self, ast: &SynFile, _file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for item in &ast.items {
            match item {
                Item::Use(_) => {
                    // Skip use statements from modules, we handle imports at the top level
                    continue;
                }
                Item::Struct(_) | Item::Impl(_) | Item::Fn(_) | Item::Trait(_) | Item::Enum(_) => {
                    // Include all public items from modules
                    self.bundled_items.push(item.clone());
                }
                _ => {
                    // Include other items too
                    self.bundled_items.push(item.clone());
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