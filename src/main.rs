use fltk::{app, button::Button, input::Input, prelude::*, window::Window, frame::Frame};
use rand::Rng;

#[derive(Clone, Copy)]
enum Cmd {
    Add, Test, Answer, InitTest,
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
    let mut store = load_store().unwrap();
    let mut data: Vec<WordPair> = vec![];
    let mut ra : usize = 0;



    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = Window::default()
        .with_size(600, 400)
        .center_screen()
        .with_label("window");

    let mut word = Input::new(10, 10, 200, 40, ""); // TODO add grid and tab
    let mut translated = Input::new(10, 70, 200, 40, "");
    let mut add_btn = Button::new(10, 130, 80, 40, "Create");
    let mut init_test = Button::new(10, 190, 80, 40, "Init Test");

    let mut ask_frame = Frame::new(240, 10, 200, 40, "");
    let mut answer_input = Input::new(300, 70, 200, 40, "");
    let mut answer_btn = Button::new(300, 130, 80, 40, "Answer");
    answer_btn.deactivate();
    // TODO show list of words and list of lessons

    let (s, r) = app::channel::<Cmd>();

    win.end();

    add_btn.set_callback( move |_| s.send(Cmd::Add));
    answer_btn.set_callback(move |_| s.send(Cmd::Answer));
    init_test.set_callback(move |_| s.send(Cmd::InitTest));

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
                            }
                        }
                        Cmd::Add => {
                            store.push(WordPair::new(word.value().as_str(), translated.value().as_str())); // TODO check is empty
                            write_to_store(&store);
                            word.set_value("");
                            translated.set_value("");
                        }
                        Cmd::Test => {
                            if let Some(v) = dice(0, data.len()) {
                                ra = v;
                                let current_pair = &data[ra];
                                ask_frame.set_label(&current_pair.translated);
                            } else {
                                ask_frame.set_label("There are no words left");
                                answer_btn.deactivate();
                                init_test.activate();
                            }
                        }
                        Cmd::Answer => {
                            let current_pair = &data[ra];
                            dbg!(&answer_input.value(),&current_pair.word);
                            if answer_input.value() == current_pair.word {
                                data.remove(ra); // TODO progress or smtg ??
                                answer_input.set_value("");
                                dbg!("OK");
                            } else {         // TODO score & score label (or progress????)
                                answer_input.set_value("");
                                dbg!("BAD");
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

fn load_store() -> Option<Vec<WordPair>> {
    let mut storage = vec![];
    let data = std::fs::read_to_string("data.db").unwrap();
    for line in data.lines() {
        let mut chunks = line.splitn(2,'-');
        let word = chunks.next().unwrap();
        let translated = chunks.next().unwrap();
        storage.push(WordPair::new(word, translated));
    }
    Some(storage)
}

fn write_to_store(data: &Vec<WordPair>) {
    for word_pair in data {
        std::fs::write("data.db", format!("{}-{}", word_pair.word, word_pair.translated));
    }
}
