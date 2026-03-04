use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub exclude_patterns: Vec<String>,
}

impl Config {
    pub fn default() -> Self {
        Self {
            exclude_patterns: vec![
                "node_modules".to_string(),
                "dist".to_string(),
                "build".to_string(),
                ".git".to_string(),
                ".next".to_string(),
                ".nuxt".to_string(),
                "coverage".to_string(),
                ".cache".to_string(),
                ".DS_Store".to_string(),
                "*.log".to_string(),
            ],
        }
    }

    pub fn load_from_file(path: &str) -> Option<Self> {
        let config_path = Path::new(path).join(".easyzip.toml");
        if let Ok(content) = fs::read_to_string(config_path) {
            toml::from_str(&content).ok()
        } else {
            None
        }
    }

    pub fn should_exclude(&self, path: &str) -> bool {
        self.exclude_patterns.iter().any(|pattern| {
            if pattern.contains('*') {
                glob::Pattern::new(pattern)
                    .ok()
                    .map(|p| p.matches(path))
                    .unwrap_or(false)
            } else {
                path.contains(pattern)
            }
        })
    }
}
