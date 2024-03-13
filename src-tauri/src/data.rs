use crate::keyboard_layout::KeyboardLayout;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProfilGame {
    pub name: String,
    pub keyboard_layout: KeyboardLayout
}

impl ProfilGame {
    pub fn new(name: &str) -> ProfilGame {
        ProfilGame {
            name: name.to_string(),
            keyboard_layout: KeyboardLayout::new()
        }
    }
}


impl Default for ProfilGame {
    fn default() -> Self {
        ProfilGame {
            name: String::new(),
            keyboard_layout: KeyboardLayout::new()
        }
    }
}

