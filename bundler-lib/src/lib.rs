/*
 * AST-based Code Bundler for Competitive Programming
 * Author: Aryan Singh <aryan.singh.iiitl@gmail.com>, Claude Sonnet 4.0
 * License: MIT
 */

pub mod ast_bundler;
pub mod file_resolver;
pub mod filename_generator;

pub use ast_bundler::AstBundler;
pub use file_resolver::FileResolver;
pub use filename_generator::FilenameGenerator;

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use syn::{File as SynFile, Item, ItemConst};

#[derive(Debug)]
pub struct BundlerConfig {
    pub main_file: PathBuf,
    pub lib_root: PathBuf,
    pub output_dir: PathBuf,
    pub create_versioned_copy: bool,
}

impl Default for BundlerConfig {
    fn default() -> Self {
        Self {
            main_file: PathBuf::from("src/main.rs"),
            lib_root: PathBuf::from("cp-lib/src"),
            output_dir: PathBuf::from("bundled"),
            create_versioned_copy: true,
        }
    }
}

pub struct CodeBundler {
    config: BundlerConfig,
    resolver: FileResolver,
    filename_gen: FilenameGenerator,
    #[allow(dead_code)]
    processed_files: HashSet<PathBuf>,
}

impl CodeBundler {
    pub fn new(config: BundlerConfig) -> Self {
        Self {
            resolver: FileResolver::new(&config.lib_root),
            filename_gen: FilenameGenerator::new(),
            config,
            processed_files: HashSet::new(),
        }
    }

    pub fn bundle(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        // Parse the main file
        let main_content = fs::read_to_string(&self.config.main_file)?;
        let main_ast: SynFile = syn::parse_str(&main_content)?;

        // If an explicit ID is provided, it fully determines the output filename.
        let explicit_id = self.extract_id(&main_ast);

        // Extract problem name
        let problem_name = self.extract_problem_name(&main_ast);
        
        // Generate filename
        let output_filename = if let Some(ref id) = explicit_id {
            format!("solution_{}.rs", id)
        } else if let Some(ref name) = problem_name {
            self.filename_gen.generate_filename(name)
        } else {
            format!(
                "solution_{}.rs",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs()
            )
        };

        let output_path = self.config.output_dir.join(&output_filename);

        // Create output directory
        fs::create_dir_all(&self.config.output_dir)?;

        // Bundle the code
        let bundled_code = self.bundle_ast(&main_ast)?;

        // Write to file
        fs::write(&output_path, &bundled_code)?;

        // Create generic copy unless an explicit ID was provided.
        if explicit_id.is_none() {
            let generic_path = self.config.output_dir.join("solution.rs");
            fs::write(&generic_path, &bundled_code)?;
            println!("Generic copy created: {}", generic_path.display());
        }

        println!("Code bundled successfully to: {}", output_path.display());
        println!("File size: {} bytes", bundled_code.len());
        if explicit_id.is_some() {
            println!("ID: {}", explicit_id.unwrap());
        } else if problem_name.is_some() {
            println!("Problem: {}", problem_name.unwrap());
        }

        Ok(output_filename)
    }

    fn extract_problem_name(&self, ast: &SynFile) -> Option<String> {
        for item in &ast.items {
            if let Item::Const(ItemConst { ident, expr, .. }) = item {
                if ident == "_PROBLEM" {
                    if let syn::Expr::Lit(syn::ExprLit { 
                        lit: syn::Lit::Str(lit_str), .. 
                    }) = &**expr {
                        return Some(lit_str.value());
                    }
                }
            }
        }
        None
    }

    fn extract_id(&self, ast: &SynFile) -> Option<String> {
        for item in &ast.items {
            let Item::Const(ItemConst { ident, expr, .. }) = item else {
                continue;
            };

            if ident != "ID" && ident != "_ID" {
                continue;
            }

            let raw = match &**expr {
                syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit_str), .. }) => lit_str.value(),
                syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) => lit_int.base10_digits().to_string(),
                _ => continue,
            };

            let sanitized: String = raw
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect();

            let sanitized = sanitized.trim_matches('_').to_string();
            if sanitized.is_empty() {
                continue;
            }

            return Some(sanitized);
        }

        None
    }

    fn bundle_ast(&mut self, main_ast: &SynFile) -> Result<String, Box<dyn std::error::Error>> {
        let mut bundler = AstBundler::new(&self.resolver);
        
        // Process the main file AST
        bundler.process_file_ast(main_ast, &self.config.main_file)?;
        
        // Generate the bundled code
        let bundled = bundler.generate_bundled_code()?;
        
        Ok(bundled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn uses_id_for_output_filename_without_timestamp() {
        let dir = tempdir().unwrap();
        let main_file = dir.path().join("main.rs");
        let out_dir = dir.path().join("out");

        std::fs::write(
            &main_file,
            "const ID: &str = \"ABC-123\";\nfn main() {}\n",
        )
        .unwrap();

        let config = BundlerConfig {
            main_file,
            // lib_root doesn't matter for this test.
            lib_root: dir.path().to_path_buf(),
            output_dir: out_dir.clone(),
            create_versioned_copy: false,
        };

        let mut bundler = CodeBundler::new(config);
        let filename = bundler.bundle().unwrap();

        assert_eq!(filename, "solution_ABC_123.rs");
        assert!(out_dir.join("solution_ABC_123.rs").exists());
        assert!(!out_dir.join("solution.rs").exists());
    }
}