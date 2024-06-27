#![windows_subsystem = "windows"]

use fltk::{
    app,
    button::Button,
    enums::{Color, Key, Shortcut},
    frame::Frame,
    group::{Group, Pack, Scroll, Tabs},
    image,
    input::Input,
    menu,
    prelude::*,
    window::Window,
};
use fltk_theme::{color_themes, ColorTheme};

mod constants;
mod entity;
mod functions;

use crate::constants::*;
use crate::entity::*;
use crate::functions::*;

// TODO lessons system

fn main() {
    let (s, r) = app::channel::<Cmd>();
    let mut store = load_store().unwrap_or_default();
    let mut data: Vec<WordPair> = vec![];
    let mut ra: usize = 0;
    let mut hint_count = 0;
    let mut fail_count = 0;

    // APP
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    if !load_settings() {
        let _theme = ColorTheme::new(color_themes::BLACK_THEME).apply();
    }

    // WINDOW
    let mut win = Window::default()
        .with_size(APP_WIDTH, APP_HEIGHT)
        .center_screen()
        .with_label(" LWT v.0.2.1");
    let win_img = image::SvgImage::from_data(APP_LOGO).ok();
    win.set_icon(win_img);

    // TAB
    let tab = Tabs::new(0, 0, APP_WIDTH, APP_HEIGHT, "");

    // ADD WORD GROUP
    let add_group = Group::default()
        .with_size(APP_WIDTH, APP_HEIGHT - TAB_BAR_HEIGHT)
        .with_label("  Add  ")
        .center_of(&tab);

    // add controls
    let mut total_lbl = Frame::new(0, 30, APP_WIDTH, 40, "");
    total_lbl.set_label(&format!("Total: {}", &store.len()));
    let mut _input_word_lbl = Frame::new(0, 80, APP_WIDTH, 40, "Word:");
    let mut word = Input::new(10, 120, APP_WIDTH - 20, 60, "");
    let _input_translated_lbl = Frame::new(0, 180, APP_WIDTH, 40, "Translated:");
    let mut translated = Input::new(10, 220, APP_WIDTH - 20, 60, "");
    let mut add_btn = Button::new(80, 320, 240, 60, "Create");

    // add controls settings
    word.set_text_color(Color::from_hex(0x8a8a8a));
    word.set_text_size(20);
    translated.set_text_color(Color::from_hex(0x8a8a8a));
    translated.set_text_size(20);
    add_btn.set_color(Color::from_hex(0xe14b00));
    add_btn.set_label_color(Color::from_hex(0xffffff));
    add_btn.set_label_size(18);
    add_btn.set_shortcut(Shortcut::from_key(Key::Enter));
    add_btn.set_tooltip("  Enter  ");

    add_group.end();

    // TEST CONTROLS GROUP
    let test_group = Group::new(10, 40, APP_WIDTH, APP_HEIGHT - TAB_BAR_HEIGHT, "  Train  ");

    let mut word_count_frame = Frame::new(0, 30, APP_WIDTH, 40, "");
    let mut score_frame = Frame::new(0, 70, APP_WIDTH, 40, "");
    let mut ask_frame = Frame::new(0, 110, APP_WIDTH, 40, "");
    ask_frame.set_label_size(20);
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
    init_test.set_label_size(18);
    init_test.set_shortcut(Shortcut::Alt | 'r');
    init_test.set_tooltip("   Alt R   ");
    ask_frame.set_color(Color::from_hex(0x282828));
    ask_frame.set_label_color(Color::from_hex(0xffffff));
    // answer_input.set_color(Color::from_hex(0x282828));
    answer_input.set_text_color(Color::from_hex(0x8a8a8a));
    answer_input.set_text_size(20);
    hint_frame.set_color(Color::from_hex(0x282828));
    hint_frame.set_label_color(Color::from_hex(0xffffff));

    hint_btn.set_color(Color::from_hex(0x00aa00));
    hint_btn.set_label_color(Color::from_hex(0xffffff));
    hint_btn.set_label_size(18);
    hint_btn.set_shortcut(Shortcut::Alt | 'h');
    hint_btn.set_tooltip("   Alt H   ");
    answer_btn.set_color(Color::from_hex(0x55557f));
    answer_btn.set_label_color(Color::from_hex(0xffffff));
    answer_btn.set_label_size(18);
    answer_btn.set_shortcut(Shortcut::from_key(Key::Enter));
    answer_btn.set_tooltip("   Enter   ");

    test_group.end();

    // WORD LIST GROUP
    let mut words_list_group = Group::new(
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
        list_btn.set_color(Color::from_hex(0x1d7385));
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

    // THEME SETTINGS GROUP
    let theme_goup = Group::default()
        .with_size(APP_WIDTH, APP_HEIGHT - TAB_BAR_HEIGHT)
        .with_label(" Themes ")
        .center_of(&tab);

    let mut theme_lbl_frame = Frame::new(0, 40, APP_WIDTH, 40, "");
    theme_lbl_frame.set_label("Choose Theme");
    let mut theme_choice = menu::Choice::default()
        .with_label("")
        .with_size(APP_WIDTH - 20, 60)
        .with_pos(10, 80);
    theme_choice.add_choice("DARK|BLACK|GRAY|SHAKE|TAN");
    theme_goup.end();

    tab.end();

    answer_btn.deactivate();
    hint_btn.deactivate();
    if store.is_empty() {
        init_test.deactivate();
    }

    // TODO show list of words and list of lessons
    win.end();

    // CALLBACKS
    add_btn.set_callback(move |_| s.send(Cmd::Add));
    answer_btn.set_callback(move |_| s.send(Cmd::Answer));
    init_test.set_callback(move |_| s.send(Cmd::InitTest));
    hint_btn.set_callback(move |_| s.send(Cmd::Hint));
    theme_choice.set_callback(|c| {
        match c.value() {
            0 => ColorTheme::new(color_themes::DARK_THEME).apply(),
            1 => ColorTheme::new(color_themes::BLACK_THEME).apply(),
            2 => ColorTheme::new(color_themes::GRAY_THEME).apply(),
            3 => ColorTheme::new(color_themes::SHAKE_THEME).apply(),
            4 => ColorTheme::new(color_themes::TAN_THEME).apply(),
            _ => unreachable!(),
        }
        save_settings(c.value())
    });

    win.show();

    // MAIN LOOP
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
                        write_to_store(&store);
                        total_lbl.set_label(&format!("Total: {}", &store.len()));

                        words_list_group.redraw();

                        word_count_frame.set_label(&format!(
                            "Words: {}/{}",
                            data.len(),
                            store.len()
                        ));
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
                            init_test.deactivate();
                            answer_btn.activate();
                            hint_btn.activate();
                            s.send(Cmd::Test);
                        }
                    }
                    Cmd::Add => {
                        if word.value().len() > 1 && translated.value().len() > 1 {
                            let new_wp =
                                WordPair::new(word.value().as_str(), translated.value().as_str());
                            store.push(new_wp.clone());
                            total_lbl.set_label(&format!("Total: {}", &store.len()));
                            let mut new_btn = Button::new(100, 100, 220, 40, ""); // TODO dry closure
                            new_btn.set_color(Color::from_hex(0x1d7385));
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
                            app::set_focus(&word);
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
                        app::set_focus(&answer_input);
                    }
                    Cmd::Answer => {
                        let current_pair = &data[ra];
                        if answer_input.value() == current_pair.word {
                            data.remove(ra);
                            answer_input.set_value("");
                            score_frame.set_label_color(Color::from_hex(0x2a7d04));
                            score_frame.set_label("GOOD!");
                        } else {
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
