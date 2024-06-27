use uuid::Uuid;

#[derive(Copy, Clone)]
pub enum Cmd {
    Add,
    Test,
    Answer,
    InitTest,
    Hint,
    Del(Uuid),
}

#[derive(Debug, Clone)]
pub struct WordPair {
    pub word: String,
    pub translated: String,
    pub id: Uuid,
}

impl WordPair {
    pub fn new(word: &str, translated: &str) -> Self {
        {
            WordPair {
                word: word.to_owned(),
                translated: translated.to_owned(),
                id: Uuid::new_v4(),
            }
        }
    }
}
