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
            ],
        }
    }

    pub fn should_exclude(&self, path: &str) -> bool {
        self.exclude_patterns.iter().any(|pattern| {
            path.contains(pattern)
        })
    }
}
