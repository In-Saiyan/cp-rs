/*
 * AST-based bundler binary
 * Author: Aryan Singh <aryan.singh.iiitl@gmail.com>
 * License: MIT
 */

use bundler_lib::{BundlerConfig, CodeBundler};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("AST-based Code Bundler v0.1.0");

    let config = BundlerConfig {
        main_file: PathBuf::from("src/main.rs"),
        lib_root: PathBuf::from("cp-lib/src"),
        output_dir: PathBuf::from("bundled"),
        create_versioned_copy: true,
    };

    let mut bundler = CodeBundler::new(config);

    match bundler.bundle() {
        Ok(filename) => {
            println!("Bundle complete: {}", filename);

            // Verify the bundled code compiles
            println!("Verifying bundled code...");
            let compile_result = std::process::Command::new("rustc")
                .args(&[
                    "bundled/solution.rs",
                    "-o",
                    "bundled/solution_test",
                    "--allow",
                    "warnings",
                ])
                .status()?;

            if compile_result.success() {
                println!("Bundled code compiles successfully!");

                // Clean up test binary
                let _ = std::fs::remove_file("bundled/solution_test");
            } else {
                println!("WARNING: Bundled code has compilation issues");
            }
        }
        Err(e) => {
            eprintln!("ERROR: Bundling failed: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
