use fltk_theme::{color_themes, ColorTheme};
use rand::Rng;
use uuid::Uuid;

use crate::WordPair;

pub fn dice(from: usize, to: usize) -> Option<usize> {
    if to == 0 {
        return None;
    }
    Some(rand::thread_rng().gen_range(from..to))
}

pub fn load_store() -> Vec<WordPair> {
    // TODO error handling with fltk msgbox
    let mut storage = vec![];
    let data = std::fs::read_to_string("data.db").unwrap_or(String::new());
    if data.is_empty() {
        return storage;
    }
    for line in data.lines() {
        let mut chunks = line.splitn(3, '|');
        let word = chunks
            .next()
            .expect("Failed to get splited data 'world' at App::load_store");
        let translated = chunks
            .next()
            .expect("Failed to splited data 'translated' at App::load_store");
        let id = chunks
            .next()
            .expect("Failed to splited data 'id' at App::load_store");
        let mut wp = WordPair::new(word, translated);
        wp.id = Uuid::parse_str(id).unwrap();
        storage.push(WordPair::new(word, translated));
    }
    storage
}

pub fn write_to_store(data: &Vec<WordPair>) {
    // TODO error handling
    if !std::path::Path::new("data.db").exists() {
        std::fs::File::create("data.db").expect("Cannot create file at write_to_store");
    }
    let mut content = String::new();
    for word_pair in data {
        content.push_str(&format!(
            "{}|{}|{}\n",
            word_pair.word, word_pair.translated, word_pair.id
        ));
    }
    std::fs::write("data.db", content).expect("Failed to write data at write_to_store");
}

pub fn save_settings(data: i32) {
    // TODO error handling
    if !std::path::Path::new("settings.ini").exists() {
        std::fs::File::create("settings.ini").expect("Cannot create file at write_to_store");
        // TODO
    }
    std::fs::write("settings.ini", &format!("{}", data))
        .expect("Failed to write data at write_to_store"); // TODO
}

pub fn load_settings() -> bool {
    if let Ok(theme_id) = std::fs::read_to_string("settings.ini") {
        if let Ok(id) = theme_id.trim().parse::<i32>() {
            match id {
                0 => ColorTheme::new(color_themes::DARK_THEME).apply(),
                1 => ColorTheme::new(color_themes::BLACK_THEME).apply(),
                2 => ColorTheme::new(color_themes::GRAY_THEME).apply(),
                3 => ColorTheme::new(color_themes::SHAKE_THEME).apply(),
                4 => ColorTheme::new(color_themes::TAN_THEME).apply(),
                _ => unreachable!(),
            }
            return true;
        };
    };
    false
}
