use fltk::{
    app, button::Button, enums::Color, enums::Key, enums::Shortcut, frame::Frame, group::Group,
    group::Tabs, input::Input, prelude::*, window::Window,
};
use rand::Rng;

#[derive(Clone, Copy)]
enum Cmd {
    Add,
    Test,
    Answer,
    InitTest,
    Hint,
}

#[derive(Debug, Clone)]
struct WordPair {
    word: String,
    translated: String,
}

impl WordPair {
    fn new(word: &str, translated: &str) -> Self {
        {
            WordPair {
                word: word.to_owned(),
                translated: translated.to_owned(),
            }
        }
    }
}
// TODO lessons system
fn main() {
    // TODO save and load data from file
    let mut store = load_store();
    let mut data: Vec<WordPair> = vec![];
    let mut ra: usize = 0;

    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    app::background(0x36, 0x39, 0x36);
    let mut win = Window::default()
        .with_size(600, 400)
        .center_screen()
        .with_label("word-trainer v0.1.2");
    win.set_color(Color::from_hex(0x363636));

    let mut tab = Tabs::new(10, 10, 580, 380, "");
    tab.set_color(Color::Blue);

    let mut add_group = Group::default()
        .with_size(580, 320)
        .with_label("Add")
        .center_of(&tab);
    add_group.set_color(Color::from_hex(0x363636));
    // add controls
    let mut word = Input::new(150, 100, 300, 40, "word:");
    let mut translated = Input::new(150, 150, 300, 40, "translated:");
    let mut add_btn = Button::new(260, 200, 80, 40, "Create");

    // add controls settings
    word.set_color(Color::from_hex(0x282828));
    word.set_text_color(Color::from_hex(0xffffff));
    translated.set_color(Color::from_hex(0x282828));
    translated.set_text_color(Color::from_hex(0xffffff));
    add_btn.set_color(Color::from_hex(0x555500));
    add_btn.set_label_color(Color::from_hex(0xffffff));

    add_group.end();

    let mut test_group = Group::new(10, 40, 540, 340, "Test");
    test_group.set_color(Color::from_hex(0x363636));

    // test controls
    let mut score_frame = Frame::new(10, 50, 580, 60, "");
    let mut init_test = Button::new(50, 150, 80, 40, "Init Test");
    let mut ask_frame = Frame::new(200, 100, 200, 60, "");
    let mut answer_input = Input::new(150, 150, 300, 40, "");
    let mut answer_btn = Button::new(200, 200, 80, 40, "Answer");
    let mut hint_btn = Button::new(300, 200, 80, 40, "Hint");
    let mut hint_frame = Frame::new(250, 250, 100, 50, "");

    // test controls settings
    init_test.set_color(Color::from_hex(0x555500));
    init_test.set_label_color(Color::from_hex(0xffffff));
    ask_frame.set_color(Color::from_hex(0x282828));
    ask_frame.set_label_color(Color::from_hex(0xffffff));
    answer_input.set_color(Color::from_hex(0x282828));
    answer_input.set_text_color(Color::from_hex(0xffffff));
    hint_frame.set_color(Color::from_hex(0x282828));
    hint_frame.set_label_color(Color::from_hex(0xffffff));

    hint_btn.set_color(Color::from_hex(0x00aa00));
    hint_btn.set_label_color(Color::from_hex(0xffffff));
    answer_btn.set_color(Color::from_hex(0x555500));
    answer_btn.set_label_color(Color::from_hex(0xffffff));
    answer_btn.set_shortcut(Shortcut::from_key(Key::Enter));

    test_group.end();

    tab.end();

    answer_btn.deactivate();
    hint_btn.deactivate();
    if store.is_empty() {
        init_test.deactivate();
    }

    // TODO show list of words and list of lessons

    let (s, r) = app::channel::<Cmd>();

    win.end();

    add_btn.set_callback(move |_| s.send(Cmd::Add));
    answer_btn.set_callback(move |_| s.send(Cmd::Answer));
    init_test.set_callback(move |_| s.send(Cmd::InitTest));
    hint_btn.set_callback(move |_| s.send(Cmd::Hint));

    win.show();
    while app.wait() {
        match r.recv() {
            Some(command) => {
                match command {
                    Cmd::InitTest => {
                        data = store.clone();
                        if let Some(v) = dice(0, data.len()) {
                            ra = v;
                            s.send(Cmd::Test);
                            init_test.deactivate();
                            answer_btn.activate();
                            hint_btn.activate();
                        }
                    }
                    Cmd::Add => {
                        if word.value().len() > 1 && translated.value().len() > 1 {
                            store.push(WordPair::new(
                                word.value().as_str(),
                                translated.value().as_str(),
                            ));
                            write_to_store(&store);
                            word.set_value("");
                            translated.set_value("");
                            init_test.activate();
                        }
                    }
                    Cmd::Test => {
                        hint_frame.set_label("");
                        if let Some(v) = dice(0, data.len()) {
                            ra = v;
                            let current_pair = &data[ra];
                            ask_frame.set_label(&current_pair.translated);
                            app::set_focus(&answer_input);
                        } else {
                            ask_frame.set_label("There are no words left");
                            answer_btn.deactivate();
                            hint_btn.deactivate();
                            init_test.activate();
                            score_frame.set_label("");
                            app::set_focus(&init_test);
                        }
                    }
                    Cmd::Hint => {
                        let current_pair = &data[ra];
                        hint_frame.set_label(&current_pair.word);
                    }
                    Cmd::Answer => {
                        let current_pair = &data[ra];
                        dbg!(&answer_input.value(), &current_pair.word);
                        if answer_input.value() == current_pair.word {
                            data.remove(ra); // TODO progress or smtg ??
                            answer_input.set_value("");
                            score_frame.set_label_color(Color::from_hex(0xaeee00));
                            score_frame.set_label(&format!(
                                "Good! {} word of {} left",
                                data.len(),
                                store.len()
                            ));
                        } else {
                            // TODO score & score label (or progress????)
                            answer_input.set_value("");
                            score_frame.set_label_color(Color::from_hex(0xff0000));
                            score_frame.set_label(&format!(
                                "Bad! {} word of {} left",
                                data.len(),
                                store.len()
                            ));
                        }
                        app::set_focus(&answer_input);
                        s.send(Cmd::Test);
                    }
                }
            }
            _ => {}
        }
    }
}

fn dice(from: usize, to: usize) -> Option<usize> {
    if to == 0 {
        return None;
    }
    Some(rand::thread_rng().gen_range(from..to))
}

fn load_store() -> Vec<WordPair> {
    // TODO error handling
    let mut storage = vec![];
    let data = std::fs::read_to_string("data.db").unwrap_or(String::new());
    if data.is_empty() {
        return storage;
    }
    for line in data.lines() {
        let mut chunks = line.splitn(2, '-');
        let word = chunks.next().unwrap();
        let translated = chunks.next().unwrap();
        storage.push(WordPair::new(word, translated));
    }
    storage
}

fn write_to_store(data: &Vec<WordPair>) {
    // TODO error handling
    if !std::path::Path::new("data.db").exists() {
        std::fs::File::create("data.db").expect("Cannot create file at write_to_store");
    }
    let mut content = String::new();
    for word_pair in data {
        content.push_str(&format!("{}-{}\n", word_pair.word, word_pair.translated));
    }
    std::fs::write("data.db", content).expect("Failed to write data at write_to_store");
    // !!!
}
