use fltk::{
    app, button::Button, enums::Color, enums::Key, enums::Shortcut, frame::Frame, group::Group,
    group::Pack, group::Scroll, group::Tabs, input::Input, prelude::*, window::Window,
};
use rand::Rng;
use uuid::Uuid;

// constants
// TODO make mod
const APP_WIDTH: i32 = 400;
const APP_HEIGHT: i32 = 600;
const TAB_BAR_HEIGHT: i32 = 60;

#[derive(Copy, Clone)]
enum Cmd {
    Add,
    Test,
    Answer,
    InitTest,
    Hint,
    Del(Uuid),
}

#[derive(Debug, Clone)]
struct WordPair {
    word: String,
    translated: String,
    id: Uuid,
}

impl WordPair {
    fn new(word: &str, translated: &str) -> Self {
        {
            WordPair {
                word: word.to_owned(),
                translated: translated.to_owned(),
                id: Uuid::new_v4(),
            }
        }
    }
}
// TODO lessons system
fn main() {
    let (s, r) = app::channel::<Cmd>();

    let mut store = load_store();
    let mut data: Vec<WordPair> = vec![];
    let mut ra: usize = 0;
    let mut hint_count = 0;
    let mut fail_count = 0;

    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    app::background(0x36, 0x39, 0x36);
    let mut win = Window::default()
        .with_size(APP_WIDTH, APP_HEIGHT)
        .center_screen()
        .with_label("word-trainer v0.1.3");
    win.set_color(Color::from_hex(0x363636));

    let mut tab = Tabs::new(0, 0, APP_WIDTH, APP_HEIGHT, "");
    tab.set_color(Color::Blue);

    let add_group = Group::default()
        .with_size(APP_WIDTH, APP_HEIGHT - TAB_BAR_HEIGHT)
        .with_label("  Add  ")
        .center_of(&tab);
    //add_group.set_color(Color::from_hex(0x363636));
    // add controls
    let mut total_lbl = Frame::new(0, 30, APP_WIDTH, 40, "");
    total_lbl.set_label(&format!("Total: {}", &store.len()));
    let _input_word_lbl = Frame::new(0, 80, APP_WIDTH, 40, "Word:");
    let mut word = Input::new(10, 120, APP_WIDTH - 20, 60, "");
    let _input_translated_lbl = Frame::new(0, 180, APP_WIDTH, 40, "Translated:");
    let mut translated = Input::new(10, 220, APP_WIDTH - 20, 60, "");
    let mut add_btn = Button::new(80, 320, 240, 60, "Create");

    // add controls settings
    word.set_color(Color::from_hex(0x282828));
    word.set_text_color(Color::from_hex(0xffffff));
    translated.set_color(Color::from_hex(0x282828));
    translated.set_text_color(Color::from_hex(0xffffff));
    add_btn.set_color(Color::from_hex(0xe14b00));
    add_btn.set_label_color(Color::from_hex(0xffffff));

    add_group.end();

    let mut test_group = Group::new(10, 40, APP_WIDTH, APP_HEIGHT - TAB_BAR_HEIGHT, "  Train  ");
    test_group.set_color(Color::from_hex(0x363636));

    // test controls
    let mut word_count_frame = Frame::new(0, 30, APP_WIDTH, 40, "");
    let mut score_frame = Frame::new(0, 70, APP_WIDTH, 40, "");
    let mut ask_frame = Frame::new(0, 110, APP_WIDTH, 40, "");
    let mut answer_input = Input::new(10, 150, APP_WIDTH - 20, 60, "");
    let mut init_test = Button::new(80, 230, 240, 60, "Start");
    let mut answer_btn = Button::new(80, 310, 240, 60, "Answer");
    let mut hint_btn = Button::new(80, 390, 240, 60, "Hint");
    let mut hint_frame = Frame::new(0, 470, APP_WIDTH, 40, "");

    let mut fail_frame = Frame::new(0, APP_HEIGHT - 40, APP_WIDTH / 2, 40, "Failed: 0");
    let mut hint_count_frame = Frame::new(
        APP_WIDTH / 2,
        APP_HEIGHT - 40,
        APP_WIDTH / 2,
        40,
        "Hints usage: 0",
    );

    // test controls settings
    word_count_frame.set_label(&format!("Words: {}/{}", data.len(), store.len()));
    init_test.set_color(Color::from_hex(0x7f00be));
    init_test.set_label_color(Color::from_hex(0xffffff));
    ask_frame.set_color(Color::from_hex(0x282828));
    ask_frame.set_label_color(Color::from_hex(0xffffff));
    answer_input.set_color(Color::from_hex(0x282828));
    answer_input.set_text_color(Color::from_hex(0xffffff));
    hint_frame.set_color(Color::from_hex(0x282828));
    hint_frame.set_label_color(Color::from_hex(0xffffff));

    hint_btn.set_color(Color::from_hex(0x00aa00));
    hint_btn.set_label_color(Color::from_hex(0xffffff));
    answer_btn.set_color(Color::from_hex(0x55557f));
    answer_btn.set_label_color(Color::from_hex(0xffffff));
    answer_btn.set_shortcut(Shortcut::from_key(Key::Enter));

    test_group.end();

    let words_list_group = Group::new(
        10,
        40,
        APP_WIDTH,
        APP_HEIGHT - TAB_BAR_HEIGHT,
        "  Words list  ",
    );
    let _scroll = Scroll::new(10, 40, APP_WIDTH - 20, APP_HEIGHT - TAB_BAR_HEIGHT, "");
    let mut pack = Pack::new(50, 40, APP_WIDTH - 120, APP_HEIGHT - 40, "");
    pack.set_spacing(5);
    let mut hide_btn = Button::new(0, 0, 0, 0, "");
    hide_btn.hide();
    for item in store.clone() {
        let mut list_btn = Button::new(0, 100, 220, 40, "");
        list_btn.set_label(&item.word);
        list_btn.set_color(Color::from_hex(0x29213b));
        list_btn.set_label_color(Color::from_hex(0xccdfd9));
        list_btn.set_callback(move |b| {
            b.hide();
            s.send(Cmd::Del(item.id));
        });
        pack.add(&list_btn);
    }
    pack.end();
    _scroll.end();
    words_list_group.end();

    tab.end();

    answer_btn.deactivate();
    hint_btn.deactivate();
    if store.is_empty() {
        init_test.deactivate();
    }

    // TODO show list of words and list of lessons

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
                    Cmd::Del(v) => {
                        let mut new_store = vec![];
                        for curr in store {
                            if curr.id != v {
                                new_store.push(curr);
                            }
                        }
                        store = new_store;
                        pack.remove_by_index(1);
                        write_to_store(&store);
                        total_lbl.set_label(&format!("Total: {}", &store.len()));
                    }
                    Cmd::InitTest => {
                        hint_count = 0;
                        fail_count = 0;
                        hint_count_frame.set_label(&format!("Hints usage: {}", hint_count));
                        fail_frame.set_label(&format!("Failed: {}", fail_count));
                        data = store.clone();
                        word_count_frame.set_label(&format!(
                            "Words: {}/{}",
                            data.len(),
                            store.len()
                        ));
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
                            let new_wp =
                                WordPair::new(word.value().as_str(), translated.value().as_str());
                            store.push(new_wp.clone());
                            total_lbl.set_label(&format!("Total: {}", &store.len()));
                            let mut new_btn = Button::new(100, 100, 220, 40, ""); // TODO dry closure
                            new_btn.set_color(Color::from_hex(0x29213b));
                            new_btn.set_label_color(Color::from_hex(0xccdfd9));
                            new_btn.set_label(&new_wp.word);
                            new_btn.set_callback(move |b| {
                                // refactor this !
                                b.hide();
                                s.send(Cmd::Del(new_wp.id));
                            });
                            pack.add(&new_btn);
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
                            ask_frame.set_label("There are no words left.");
                            answer_btn.deactivate();
                            hint_btn.deactivate();
                            init_test.activate();
                            score_frame.set_label("");
                            app::set_focus(&init_test);
                        }
                    }
                    Cmd::Hint => {
                        hint_count += 1;
                        let current_pair = &data[ra];
                        hint_frame.set_label(&current_pair.word);
                        hint_count_frame.set_label(&format!("Hints usage: {}", hint_count));
                    }
                    Cmd::Answer => {
                        let current_pair = &data[ra];
                        // DEBUG
                        //dbg!(&answer_input.value(), &current_pair.word);
                        if answer_input.value() == current_pair.word {
                            data.remove(ra); // TODO progress or smtg ??
                            answer_input.set_value("");
                            score_frame.set_label_color(Color::from_hex(0xaeee00));
                            score_frame.set_label("GOOD!");
                        } else {
                            // TODO score & score label (or progress????)
                            fail_count += 1;
                            fail_frame.set_label(&format!("Failed: {}", fail_count));
                            answer_input.set_value("");
                            score_frame.set_label_color(Color::from_hex(0xff0000));
                            score_frame.set_label("WRONG!");
                        }

                        word_count_frame.set_label(&format!(
                            "Words: {}/{}",
                            data.len(),
                            store.len()
                        ));
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

fn write_to_store(data: &Vec<WordPair>) {
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
