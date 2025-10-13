// build.rs - Run before compilation to create bundled version
use std::process::Command;

fn main() {
    // Only run bundling in release mode or when explicitly requested
    if std::env::var("PROFILE").unwrap_or_default() == "release" || 
       std::env::var("BUNDLE_CODE").is_ok() {
        
        println!("cargo:rerun-if-changed=src/main.rs");
        println!("cargo:rerun-if-changed=cp-lib/src/");
        
        // Run the bundler
        let _ = Command::new("cargo")
            .args(&["run", "--bin", "bundler"])
            .status();
    }
}