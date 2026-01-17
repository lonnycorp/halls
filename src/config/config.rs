use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum::{EnumCount, IntoEnumIterator};
use url::Url;
use winit::keyboard::KeyCode;

use super::ConfigControl;

const CONFIG_PATH: &str = "halls/config.json";
const DEFAULT_URL: &str = "https://lonnycorp.github.io/halls-nexus";

fn config_path() -> PathBuf {
    let dir = std::env::var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap();
            return PathBuf::from(home).join(".local/state");
        });
    return dir.join(CONFIG_PATH);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub volume: f32,
    pub mouse_sensitivity: f32,
    pub default_url: Url,
    controls: [KeyCode; ConfigControl::COUNT],
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        let mut config = fs::read_to_string(&path)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_else(Self::new);
        config.volume = config.volume.clamp(0.0, 1.0);
        return config;
    }

    pub fn save(&self) {
        let path = config_path();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, serde_json::to_string(self).unwrap()).unwrap();
    }

    pub fn new() -> Self {
        let controls: [KeyCode; ConfigControl::COUNT] = ConfigControl::iter()
            .map(|c| c.default_key_code())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        return Self {
            volume: 1.0,
            mouse_sensitivity: 1.0,
            default_url: Url::parse(DEFAULT_URL).unwrap(),
            controls,
        };
    }

    pub fn keycode_get(&self, control: ConfigControl) -> KeyCode {
        return self.controls[control as usize];
    }

    pub fn keycode_set(&mut self, control: ConfigControl, key: KeyCode) {
        self.controls[control as usize] = key;
    }
}
