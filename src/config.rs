use serde_derive::{Deserialize, Serialize};
use std::env;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub items: Vec<Item>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    pub description: String,
    pub command: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: "作業リスト".to_string(),
            items: vec![],
        }
    }
}

impl Config {
    const FILENAME: &str = "config.toml";

    pub fn new() -> Self {
        let exe = env::current_exe().unwrap();
        let dir = Path::new(&exe).parent().unwrap();

        confy::load_path::<Self>(dir.join(Self::FILENAME)).unwrap()
    }
}
