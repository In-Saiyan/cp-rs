/*
 * File resolver for finding module files
 */

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileResolver {
    lib_root: PathBuf,
    module_cache: std::collections::HashMap<String, PathBuf>,
}

impl FileResolver {
    pub fn new(lib_root: &Path) -> Self {
        let mut resolver = Self {
            lib_root: lib_root.to_path_buf(),
            module_cache: std::collections::HashMap::new(),
        };
        
        resolver.build_cache();
        resolver
    }

    fn build_cache(&mut self) {
        if !self.lib_root.exists() {
            return;
        }

        for entry in WalkDir::new(&self.lib_root)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                if let Some(relative_path) = path.strip_prefix(&self.lib_root).ok() {
                    let key = relative_path.to_string_lossy().to_string();
                    self.module_cache.insert(key, path.to_path_buf());
                }
            }
        }
    }

    pub fn resolve_module_file(&self, module_path: &Path) -> Option<PathBuf> {
        let key = module_path.to_string_lossy().to_string();
        
        // Direct cache lookup
        if let Some(cached_path) = self.module_cache.get(&key) {
            return Some(cached_path.clone());
        }

        // Try variations
        let variations = vec![
            module_path.to_path_buf(),
            {
                let mut p = module_path.to_path_buf();
                if p.extension().is_none() {
                    p.set_extension("rs");
                }
                p
            },
            {
                let mut p = module_path.parent().unwrap_or(Path::new("")).to_path_buf();
                if let Some(file_stem) = module_path.file_stem() {
                    p.push(file_stem);
                    p.push("mod.rs");
                }
                p
            },
        ];

        for variation in variations {
            let full_path = self.lib_root.join(&variation);
            if full_path.exists() && full_path.is_file() {
                return Some(full_path);
            }

            // Also check in cache with this variation
            let var_key = variation.to_string_lossy().to_string();
            if let Some(cached_path) = self.module_cache.get(&var_key) {
                return Some(cached_path.clone());
            }
        }

        None
    }

    pub fn list_all_files(&self) -> Vec<&PathBuf> {
        self.module_cache.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_file_resolver() {
        let temp_dir = tempdir().unwrap();
        let lib_root = temp_dir.path().join("lib");
        
        // Create test structure
        fs::create_dir_all(&lib_root.join("io")).unwrap();
        fs::write(lib_root.join("lib.rs"), "").unwrap();
        fs::write(lib_root.join("io").join("mod.rs"), "").unwrap();
        fs::write(lib_root.join("io").join("scanner.rs"), "").unwrap();

        let resolver = FileResolver::new(&lib_root);
        
        assert!(resolver.resolve_module_file(Path::new("io/scanner.rs")).is_some());
        assert!(resolver.resolve_module_file(Path::new("io/mod.rs")).is_some());
        assert!(resolver.resolve_module_file(Path::new("nonexistent.rs")).is_none());
    }
}
