use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MchoseProfile {
    #[serde(flatten)]
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileEntry {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub hotkey: Option<String>,
}

pub fn get_profiles_dir() -> PathBuf {
    let mut dir = dirs::config_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
    dir.push("Ace60Switcher");
    dir.push("profiles");
    let _ = fs::create_dir_all(&dir);
    dir
}

pub fn load_profiles() -> Vec<ProfileEntry> {
    let dir = get_profiles_dir();
    let mut profiles = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if let Ok(path) = entry.path().canonicalize() {
                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                            let name = val
                                .get("data")
                                .and_then(|d| d.get("name"))
                                .and_then(|n| n.as_str())
                                .or_else(|| val.get("name").and_then(|n| n.as_str()))
                                .unwrap_or("Unnamed")
                                .to_string();
                            let id = path.file_stem().unwrap().to_string_lossy().to_string();
                            profiles.push(ProfileEntry {
                                id,
                                name,
                                path,
                                hotkey: None,
                            });
                        }
                    }
                }
            }
        }
    }
    profiles
}
