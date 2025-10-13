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

        // Extract problem name
        let problem_name = self.extract_problem_name(&main_ast);
        
        // Generate filename
        let output_filename = if let Some(ref name) = problem_name {
            self.filename_gen.generate_filename(name)
        } else {
            format!("solution_{}.rs", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs())
        };

        let output_path = self.config.output_dir.join(&output_filename);

        // Create output directory
        fs::create_dir_all(&self.config.output_dir)?;

        // Bundle the code
        let bundled_code = self.bundle_ast(&main_ast)?;

        // Write to file
        fs::write(&output_path, &bundled_code)?;
        
        // Create generic copy
        let generic_path = self.config.output_dir.join("solution.rs");
        fs::write(&generic_path, &bundled_code)?;

        println!("Code bundled successfully to: {}", output_path.display());
        println!("File size: {} bytes", bundled_code.len());
        if problem_name.is_some() {
            println!("Problem: {}", problem_name.unwrap());
        }
        println!("Generic copy created: {}", generic_path.display());

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

    fn bundle_ast(&mut self, main_ast: &SynFile) -> Result<String, Box<dyn std::error::Error>> {
        let mut bundler = AstBundler::new(&self.resolver);
        
        // Process the main file AST
        bundler.process_file_ast(main_ast, &self.config.main_file)?;
        
        // Generate the bundled code
        let bundled = bundler.generate_bundled_code()?;
        
        Ok(bundled)
    }
}