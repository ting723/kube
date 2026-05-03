use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyConfig {
    #[serde(default = "default_quit")]
    pub quit: char,
    #[serde(default = "default_help")]
    pub help: char,
    #[serde(default = "default_up")]
    pub up: char,
    #[serde(default = "default_down")]
    pub down: char,
    #[serde(default = "default_logs")]
    pub logs: char,
    #[serde(default = "default_exec")]
    pub exec: char,
    #[serde(default = "default_yaml")]
    pub yaml: char,
    #[serde(default = "default_describe")]
    pub describe: char,
    #[serde(default = "default_search")]
    pub search: char,
    #[serde(default = "default_batch")]
    pub batch: char,
    #[serde(default = "default_split")]
    pub split: char,
    #[serde(default = "default_port_forward")]
    pub port_forward: char,
    #[serde(default = "default_context")]
    pub context: char,
    #[serde(default = "default_sort")]
    pub sort: char,
    #[serde(default = "default_delete")]
    pub delete: char,
    #[serde(default = "default_language")]
    pub language: char,
    #[serde(default = "default_refresh")]
    pub refresh: char,
    #[serde(default = "default_mouse")]
    pub mouse: char,
}

fn default_quit() -> char {
    'q'
}
fn default_help() -> char {
    '?'
}
fn default_up() -> char {
    'k'
}
fn default_down() -> char {
    'j'
}
fn default_logs() -> char {
    'L'
}
fn default_exec() -> char {
    'E'
}
fn default_yaml() -> char {
    'Y'
}
fn default_describe() -> char {
    ' '
}
fn default_search() -> char {
    '/'
}
fn default_batch() -> char {
    'v'
}
fn default_split() -> char {
    'V'
}
fn default_port_forward() -> char {
    'P'
}
fn default_context() -> char {
    'C'
}
fn default_sort() -> char {
    '>'
}
fn default_delete() -> char {
    'D'
}
fn default_language() -> char {
    'I'
}
fn default_refresh() -> char {
    'R'
}
fn default_mouse() -> char {
    'M'
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            quit: 'q',
            help: '?',
            up: 'k',
            down: 'j',
            logs: 'L',
            exec: 'E',
            yaml: 'Y',
            describe: ' ',
            search: '/',
            batch: 'v',
            split: 'V',
            port_forward: 'P',
            context: 'C',
            sort: '>',
            delete: 'D',
            language: 'I',
            refresh: 'R',
            mouse: 'M',
        }
    }
}

impl KeyConfig {
    pub fn config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
        PathBuf::from(home).join(".config/kube-tui/keys.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        Self::default()
    }
}
