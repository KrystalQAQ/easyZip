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

    pub fn smart_detect(path: &str) -> Self {
        let mut config = Self::default();

        let project_path = Path::new(path);

        // React
        if project_path.join("package.json").exists() {
            if let Ok(content) = fs::read_to_string(project_path.join("package.json")) {
                if content.contains("\"react\"") {
                    config.exclude_patterns.extend(vec![
                        "build".to_string(),
                        ".cache".to_string(),
                    ]);
                }

                // Next.js
                if content.contains("\"next\"") {
                    config.exclude_patterns.extend(vec![
                        ".next".to_string(),
                        "out".to_string(),
                    ]);
                }

                // Vue
                if content.contains("\"vue\"") {
                    config.exclude_patterns.push("dist".to_string());
                }

                // Nuxt
                if content.contains("\"nuxt\"") {
                    config.exclude_patterns.extend(vec![
                        ".nuxt".to_string(),
                        ".output".to_string(),
                    ]);
                }

                // Vite
                if content.contains("\"vite\"") {
                    config.exclude_patterns.push("dist".to_string());
                }
            }
        }

        // Angular
        if project_path.join("angular.json").exists() {
            config.exclude_patterns.extend(vec![
                "dist".to_string(),
                ".angular".to_string(),
            ]);
        }

        // Rust
        if project_path.join("Cargo.toml").exists() {
            config.exclude_patterns.push("target".to_string());
        }

        // Python
        if project_path.join("requirements.txt").exists()
            || project_path.join("pyproject.toml").exists() {
            config.exclude_patterns.extend(vec![
                "__pycache__".to_string(),
                "*.pyc".to_string(),
                ".venv".to_string(),
                "venv".to_string(),
            ]);
        }

        config.exclude_patterns.sort();
        config.exclude_patterns.dedup();
        config
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
