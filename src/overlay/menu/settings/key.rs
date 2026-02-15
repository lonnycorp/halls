use std::collections::HashMap;

use winit::keyboard::Key;

pub struct MenuSettingsKeyCache {
    names: HashMap<Key, String>,
}

impl MenuSettingsKeyCache {
    pub fn new() -> Self {
        return Self {
            names: HashMap::new(),
        };
    }

    pub fn name<'a>(&'a mut self, key: &Key) -> &'a str {
        if !self.names.contains_key(key) {
            let name = match key.as_ref() {
                Key::Character(text) => {
                    let text = text.to_ascii_uppercase();
                    format!("KEY_{text}")
                }
                Key::Named(name) => {
                    let name = format!("{:?}", name).to_ascii_uppercase();
                    format!("KEY_{name}")
                }
                _ => {
                    let name = format!("{:?}", key.as_ref()).to_ascii_uppercase();
                    format!("KEY_{name}")
                }
            };

            self.names.insert(key.clone(), name);
        }

        return self.names.get(key).unwrap();
    }
}
