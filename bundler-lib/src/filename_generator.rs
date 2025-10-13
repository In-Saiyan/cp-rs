/*
 * Filename generator for problem names
 */

use regex::Regex;

pub struct FilenameGenerator {
    cleanup_regex: Regex,
}

impl FilenameGenerator {
    pub fn new() -> Self {
        Self {
            cleanup_regex: Regex::new(r"_+").unwrap(),
        }
    }

    pub fn generate_filename(&self, problem_name: &str) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let formatted_name = self.format_problem_name(problem_name);
        format!("{}_{}.rs", formatted_name, timestamp)
    }

    fn format_problem_name(&self, problem_name: &str) -> String {
        let mut filename = String::new();
        
        let chars: Vec<char> = problem_name.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            match ch {
                // Keep alphanumeric characters
                c if c.is_alphanumeric() => {
                    if c.is_alphabetic() {
                        filename.push(c.to_lowercase().next().unwrap_or(c));
                    } else {
                        filename.push(c); // Keep numbers as-is
                    }
                }
                // Handle dots specially (for "A.", "B1.", etc.)
                '.' => {
                    filename.push('.');
                    // Skip spaces after dot
                    while i + 1 < chars.len() && chars[i + 1].is_whitespace() {
                        i += 1;
                    }
                    filename.push('_');
                }
                // Replace whitespace and special chars with underscore
                _ => {
                    filename.push('_');
                }
            }
            
            i += 1;
        }
        
        // Clean up the filename
        let filename = filename.trim_end_matches('_');
        let filename = self.cleanup_regex.replace_all(filename, "_");
        
        filename.to_string()
    }
}

impl Default for FilenameGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_problem_name() {
        let generator = FilenameGenerator::new();
        
        assert_eq!(generator.format_problem_name("A. Simple Problem"), "a._simple_problem");
        assert_eq!(generator.format_problem_name("D2. Magic Powder"), "d2._magic_powder");
        assert_eq!(generator.format_problem_name("F1. Tree Cutting (Easy Version)"), "f1._tree_cutting_easy_version");
        assert_eq!(generator.format_problem_name("C. Some-Complex_Problem!"), "c._some_complex_problem");
    }

    #[test]
    fn test_generate_filename() {
        let generator = FilenameGenerator::new();
        let filename = generator.generate_filename("A. Test Problem");
        
        assert!(filename.starts_with("a._test_problem_"));
        assert!(filename.ends_with(".rs"));
        assert!(filename.len() > "a._test_problem_.rs".len()); // Should have timestamp
    }
}