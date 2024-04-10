use crate::keyboard_layout::KeyboardLayoutInfo;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProfilGame {
    pub name: String,
    pub keyboard_layout: KeyboardLayoutInfo
}

impl ProfilGame {
    pub fn new(name: &str) -> ProfilGame {
        ProfilGame {
            name: name.to_string(),
            keyboard_layout: KeyboardLayoutInfo::new().expect("Failed to get keyboard layout info")
        }
    }
}

impl Default for ProfilGame {
    fn default() -> Self {
        ProfilGame {
            name: String::new(),
            keyboard_layout: KeyboardLayoutInfo::new().expect("Failed to get keyboard layout info")
        }
    }
}