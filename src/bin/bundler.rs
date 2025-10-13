/*
 * Author: Aryan Singh <aryan.singh.iiitl@gmail.com>
 * License: MIT
 * Date: 2024-06-10
 */

use std::collections::HashSet;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use regex::Regex;

struct CodeBundler {
    #[allow(dead_code)]
    processed_modules: HashSet<String>,
}

impl CodeBundler {
    fn new() -> Self {
        Self {
            processed_modules: HashSet::new(),
        }
    }

    fn bundle_code(&mut self, main_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let main_content = fs::read_to_string(main_file)?;
        
        // Start with the bundled content
        let mut bundled_content = String::from("// Code bundled for competitive programming\n");
        bundled_content.push_str("// Generated automatically - do not edit manually\n");
        bundled_content.push_str(&format!("// Original file: {}\n", main_file));
        bundled_content.push_str(&format!("// Generated at: {}\n\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
        
        // Add standard library imports 
        bundled_content.push_str("use std::io::{self, BufRead, Cursor};\n");
        bundled_content.push_str("use std::collections::*;\n\n");
        
        // Process and expand cp_lib imports
        let expanded_content = self.expand_cp_lib_imports(&main_content)?;
        
        // Add the expanded content
        bundled_content.push_str(&expanded_content);
        
        // Write to output file
        let content_len = bundled_content.len();
        fs::write(output_file, bundled_content)?;
        
        println!("✅ Code bundled successfully to: {}", output_file);
        println!("📁 File size: {} bytes", content_len);
        Ok(())
    }

    fn expand_cp_lib_imports(&mut self, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        let mut scanner_included = false;
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Skip cp_lib imports and replace with expanded code
            if trimmed.starts_with("use cp_lib::") {
                if trimmed.contains("scanner::Scanner") && !scanner_included {
                    result.push_str(&self.get_scanner_code()?);
                    scanner_included = true;
                }
                // Skip the import line
                continue;
            }
            
            // Keep all other lines
            result.push_str(line);
            result.push('\n');
        }
        
        Ok(result)
    }

    fn get_scanner_code(&self) -> Result<String, Box<dyn std::error::Error>> {
        let scanner_path = "cp-lib/src/io/scanner.rs";
        if !std::path::Path::new(scanner_path).exists() {
            return Err("Scanner file not found. Make sure you're running from the project root.".into());
        }
        
        let content = fs::read_to_string(scanner_path)?;
        
        let mut scanner_code = String::new();
        scanner_code.push_str("// ==================== Scanner Implementation ====================\n");
        
        // Process the scanner code to remove ALL use statements since we already have them at the top
        for line in content.lines() {
            let trimmed = line.trim();
            
            // Skip ALL use statements
            if trimmed.starts_with("use ") {
                continue;
            }
            
            scanner_code.push_str(line);
            scanner_code.push('\n');
        }
        
        scanner_code.push_str("// ===============================================================\n\n");
        
        Ok(scanner_code)
    }
}

fn extract_problem_name(content: &str) -> Option<String> {
    // Look for _PROBLEM constant definition
    let re = Regex::new(r#"const\s+_PROBLEM\s*:\s*&str\s*=\s*"([^"]+)""#).unwrap();
    
    if let Some(captures) = re.captures(content) {
        if let Some(problem_name) = captures.get(1) {
            return Some(problem_name.as_str().to_string());
        }
    }
    
    None
}

fn format_filename(problem_name: &str) -> String {
    // Convert "D2. Magic Powder" to "d2._magic_powder"
    let mut filename = String::new();
    
    let chars: Vec<char> = problem_name.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        let ch = chars[i];
        
        if ch.is_alphanumeric() {
            // Keep both letters and numbers, convert letters to lowercase
            if ch.is_alphabetic() {
                filename.push(ch.to_lowercase().next().unwrap());
            } else {
                filename.push(ch); // Keep numbers as-is
            }
        } else if ch.is_whitespace() {
            filename.push('_');
        } else if ch == '.' {
            filename.push('.');
            // Skip spaces after dot
            while i + 1 < chars.len() && chars[i + 1].is_whitespace() {
                i += 1;
            }
            filename.push('_');
        } else {
            // Replace other special characters with underscore
            filename.push('_');
        }
        
        i += 1;
    }
    
    // Remove trailing underscores and clean up multiple underscores
    let filename = filename.trim_end_matches('_').to_string();
    let filename = Regex::new(r"_+").unwrap().replace_all(&filename, "_").to_string();
    
    filename
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bundler = CodeBundler::new();
    
    // Read main.rs to extract problem name
    let main_content = fs::read_to_string("src/main.rs")?;
    
    // Extract problem name or use default
    let problem_name = extract_problem_name(&main_content)
        .unwrap_or_else(|| "solution".to_string());
    
    // Generate timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Format filename
    let formatted_name = format_filename(&problem_name);
    let output_filename = format!("bundled/{}_{}.rs", formatted_name, timestamp);
    
    // Create output directory if it doesn't exist
    fs::create_dir_all("bundled")?;
    
    // Bundle the main.rs file with the formatted name
    bundler.bundle_code("src/main.rs", &output_filename)?;
    
    // Also create a generic solution.rs for convenience
    fs::copy(&output_filename, "bundled/solution.rs")?;
    println!("📋 Generic copy created: bundled/solution.rs");
    
    Ok(())
}