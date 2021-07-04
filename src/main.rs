use fltk::{app, button::Button, input::Input, prelude::*, window::Window, frame::Frame, enums::Color};
use rand::Rng;

#[derive(Clone, Copy)]
enum Cmd {
    Add, Test, Answer, InitTest, Hint,
}

#[derive(Debug, Clone)]
struct WordPair {
    word: String,
    translated: String,
}


impl WordPair {
    fn new(word: &str, translated: &str) -> Self {
        {
            WordPair{word: word.to_owned(), translated: translated.to_owned()}
        }
    }
}
// TODO lessons system
fn main() { // TODO save and load data from file
    let mut store = load_store();
    let mut data: Vec<WordPair> = vec![];
    let mut ra : usize = 0;



    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = Window::default()
        .with_size(600, 400)
        .center_screen()
        .with_label("word-trainer v0.1.2");

    let mut word = Input::new(10, 10, 200, 40, ""); // TODO add grid and tab
    let mut translated = Input::new(10, 70, 200, 40, "");
    let mut add_btn = Button::new(10, 130, 80, 40, "Create");
    let mut init_test = Button::new(10, 190, 80, 40, "Init Test");

    let mut ask_frame = Frame::new(240, 10, 200, 40, "");
    let mut answer_input = Input::new(300, 70, 200, 40, "");
    let mut answer_btn = Button::new(300, 130, 80, 40, "Answer");
    let mut hint_btn = Button::new(400, 130, 80, 40, "Hint");
    let mut hint_frame = Frame::new(300, 190, 200, 50, "" );
    let mut score_frame = Frame::new(10, 250, 580, 60, "");

    answer_btn.deactivate();
    hint_btn.deactivate();
    if store.is_empty() {
        init_test.deactivate();
    }

    // TODO show list of words and list of lessons

    let (s, r) = app::channel::<Cmd>();

    win.end();

    add_btn.set_callback( move |_| s.send(Cmd::Add));
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
                                store.push(WordPair::new(word.value().as_str(), translated.value().as_str()));
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
                            } else {
                                ask_frame.set_label("There are no words left");
                                answer_btn.deactivate();
                                hint_btn.deactivate();
                                init_test.activate();
                                score_frame.set_label("");
                            }
                        }
                        Cmd::Hint => {
                            let current_pair = &data[ra];
                            hint_frame.set_label(&current_pair.word);
                        }
                        Cmd::Answer => {
                            let current_pair = &data[ra];
                            dbg!(&answer_input.value(),&current_pair.word);
                            if answer_input.value() == current_pair.word {
                                data.remove(ra); // TODO progress or smtg ??
                                answer_input.set_value("");
                                score_frame.set_label_color(Color::from_hex(0xaeee00));
                                score_frame.set_label(&format!("Good! {} word of {} left", data.len(), store.len()));
                            } else {         // TODO score & score label (or progress????)
                                answer_input.set_value("");
                                score_frame.set_label_color(Color::from_hex(0xff0000));
                                score_frame.set_label(&format!("Bad! {} word of {} left", data.len(), store.len()));
                            }
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

fn load_store() -> Vec<WordPair> { // TODO error handling
    let mut storage = vec![];
    let data = std::fs::read_to_string("data.db").unwrap_or(String::new());
    if data.is_empty() {
        return storage;
    }
    for line in data.lines() {
        let mut chunks = line.splitn(2,'-');
        let word = chunks.next().unwrap();
        let translated = chunks.next().unwrap();
        storage.push(WordPair::new(word, translated));
    }
    storage
}

fn write_to_store(data: &Vec<WordPair>) { // TODO error handling
    if !std::path::Path::new("data.db").exists() {
        std::fs::File::create("data.db").expect("Cannot create file at write_to_store");
    }
    let mut content = String::new();
    for word_pair in data {
        content.push_str(&format!("{}-{}\n", word_pair.word, word_pair.translated));
    }
    std::fs::write("data.db", content).expect("Failed to write data at write_to_store"); // !!!
}
