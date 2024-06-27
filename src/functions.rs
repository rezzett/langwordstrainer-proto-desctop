use fltk::dialog;
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

pub fn load_store() -> Option<Vec<WordPair>> {
    let mut storage = vec![];
    let data = std::fs::read_to_string("data.db").unwrap_or(String::new());
    if data.is_empty() {
        return Some(storage);
    }
    for line in data.lines() {
        let mut chunks = line.splitn(3, '|');
        let word = chunks.next()?;
        let translated = chunks.next()?;
        let id = chunks.next()?;
        let mut wp = WordPair::new(word, translated);
        wp.id = Uuid::parse_str(id).unwrap_or_default();
        storage.push(WordPair::new(word, translated));
    }
    Some(storage)
}

pub fn write_to_store(data: &Vec<WordPair>) {
    if !std::path::Path::new("data.db").exists() {
        if !std::fs::File::create("data.db").is_ok() {
            dialog::alert(100, 100, "Cannot create file at write_to_store");
        }
    }
    let mut content = String::new();
    for word_pair in data {
        content.push_str(&format!(
            "{}|{}|{}\n",
            word_pair.word, word_pair.translated, word_pair.id
        ));
    }
    if !std::fs::write("data.db", content).is_ok() {
        dialog::alert(100, 100, "Failed to write data at write_to_store");
    }
}

pub fn save_settings(data: i32) {
    if !std::path::Path::new("settings.ini").exists() {
        if !std::fs::File::create("settings.ini").is_ok() {
            dialog::alert(100, 100, "Cannot create file to save setting")
        }
    }
    if !std::fs::write("settings.ini", &format!("{}", data)).is_ok() {
        dialog::alert(100, 100, "Failed to write data to save settings")
    }
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
