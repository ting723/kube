//! User configuration management (favorite namespaces, command history)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// User configuration stored in ~/.config/kube-tui/config.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    /// Favorite namespaces for quick switching
    pub favorite_namespaces: Vec<String>,
    /// Command history (max 100 entries)
    pub command_history: Vec<String>,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            favorite_namespaces: Vec::new(),
            command_history: Vec::new(),
        }
    }
}

impl UserConfig {
    /// Get the config file path
    pub fn config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
        PathBuf::from(home).join(".config/kube-tui/config.json")
    }

    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let path = Self::config_path();

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        let config: UserConfig = serde_json::from_str(&content)?;

        // Ensure command_history has max 100 entries
        let config = Self::limit_command_history(config);

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();

        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;

        Ok(())
    }

    /// Add a command to history (max 100 entries)
    pub fn add_command(&mut self, command: String) {
        // Don't add duplicate consecutive commands
        if self.command_history.last() == Some(&command) {
            return;
        }

        self.command_history.push(command);

        // Limit to 100 entries
        if self.command_history.len() > 100 {
            self.command_history.remove(0);
        }
    }

    /// Add or remove a favorite namespace
    pub fn toggle_favorite_namespace(&mut self, namespace: &str) {
        if self.favorite_namespaces.contains(&namespace.to_string()) {
            self.favorite_namespaces.retain(|n| n != namespace);
        } else {
            self.favorite_namespaces.push(namespace.to_string());
        }
    }

    /// Check if a namespace is favorited
    pub fn is_favorite_namespace(&self, namespace: &str) -> bool {
        self.favorite_namespaces.contains(&namespace.to_string())
    }

    /// Limit command history to 100 entries
    fn limit_command_history(mut config: Self) -> Self {
        if config.command_history.len() > 100 {
            config.command_history = config.command_history.split_off(config.command_history.len() - 100);
        }
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = UserConfig::default();
        assert!(config.favorite_namespaces.is_empty());
        assert!(config.command_history.is_empty());
    }

    #[test]
    fn test_toggle_favorite_namespace() {
        let mut config = UserConfig::default();

        // Add favorite
        config.toggle_favorite_namespace("default");
        assert!(config.is_favorite_namespace("default"));
        assert_eq!(config.favorite_namespaces.len(), 1);

        // Remove favorite
        config.toggle_favorite_namespace("default");
        assert!(!config.is_favorite_namespace("default"));
        assert_eq!(config.favorite_namespaces.len(), 0);
    }

    #[test]
    fn test_add_command() {
        let mut config = UserConfig::default();

        config.add_command("kubectl get pods".to_string());
        assert_eq!(config.command_history.len(), 1);
        assert_eq!(config.command_history[0], "kubectl get pods");

        // Add another command
        config.add_command("kubectl delete pod nginx".to_string());
        assert_eq!(config.command_history.len(), 2);
    }

    #[test]
    fn test_no_duplicate_consecutive_commands() {
        let mut config = UserConfig::default();

        config.add_command("kubectl get pods".to_string());
        config.add_command("kubectl get pods".to_string()); // Same as last

        assert_eq!(config.command_history.len(), 1); // Only one entry
    }

    #[test]
    fn test_command_history_limit() {
        let mut config = UserConfig::default();

        // Add 105 commands
        for i in 0..105 {
            config.add_command(format!("command {}", i));
        }

        // Should be limited to 100
        assert_eq!(config.command_history.len(), 100);

        // First command should be "command 5" (oldest kept)
        assert_eq!(config.command_history[0], "command 5");

        // Last command should be "command 104" (newest)
        assert_eq!(config.command_history[99], "command 104");
    }

    #[test]
    fn test_config_path() {
        let path = UserConfig::config_path();
        assert!(path.to_string_lossy().contains(".config"));
        assert!(path.to_string_lossy().contains("kube-tui"));
        assert!(path.to_string_lossy().contains("config.json"));
    }

    #[test]
    fn test_save_and_load() {
        let mut config = UserConfig::default();
        config.toggle_favorite_namespace("default");
        config.toggle_favorite_namespace("kube-system");
        config.add_command("kubectl get pods".to_string());

        // Save
        config.save().expect("Failed to save config");

        // Load
        let loaded = UserConfig::load().expect("Failed to load config");

        assert_eq!(loaded.favorite_namespaces, config.favorite_namespaces);
        assert_eq!(loaded.command_history, config.command_history);

        // Cleanup: delete test config file
        let path = UserConfig::config_path();
        if path.exists() {
            std::fs::remove_file(&path).ok();
        }
    }
}