use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorApp {
    #[error("What is this command, I never heard of it 🙃")]
    ErrorCommand,
}

// available commands
pub const HELP: [&str; 3] = ["help", "h", "?"];
pub const PLAY: [&str; 2] = ["play", "p"];
pub const DEFINITION: [&str; 2] = ["definition", "d"];
pub const SYNONYME: [&str; 2] = ["synonyme", "s"];

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct JishoResponse {
    meta: Meta,
    data: Vec<WordEntry>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Meta {
    status: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WordEntry {
    slug: String,
    is_common: bool,
    japanese: Vec<Japanese>,
    senses: Vec<Sense>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Japanese {
    word: Option<String>,
    reading: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Sense {
    english_definitions: Vec<String>,
    parts_of_speech: Vec<String>,
}
