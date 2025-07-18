#![allow(unused)]

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorApp {
    #[error("I wish learned somthing 🙂")]
    GoodBye,
    #[error("An error occured when reaching to the api")]
    ErrorApi,
    #[error("What is this command, I never heard of it 🙃")]
    ErrorCommand,
    #[error("An error has occured when trying to peak")]
    ErrorSpeak,
}

// available commands
pub const QUIT: [&str; 2] = ["exit", "e"];
pub const HELP: [&str; 3] = ["help", "h", "?"];
pub const WORD: [&str; 2] = ["word", "w"];
pub const PLAY: [&str; 2] = ["play", "p"];
pub const DEFINITION: [&str; 2] = ["definition", "d"];
pub const SYNONYME: [&str; 2] = ["synonyme", "s"];

#[derive(Debug, Deserialize)]
pub struct JishoResponse {
    meta: Meta,
    data: Vec<WordEntry>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    status: u32,
}

#[derive(Debug, Deserialize)]
pub struct WordEntry {
    slug: String,
    is_common: bool,
    japanese: Vec<Japanese>,
    senses: Vec<Sense>,
}

#[derive(Debug, Deserialize)]
pub struct Japanese {
    word: Option<String>,
    reading: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Sense {
    english_definitions: Vec<String>,
    parts_of_speech: Vec<String>,
}

impl JishoResponse {
    fn alphabet(&self, index: usize) -> Vec<&Japanese> {
        let mut ja_word: Vec<&Japanese> = Vec::new();
        for alpha in &self.data[index].japanese {
            ja_word.push(alpha);
        }

        ja_word
    }

    fn en_senses(&self, index: usize) -> Vec<&Sense> {
        let mut senses: Vec<&Sense> = Vec::new();
        for sense in &self.data[index].senses {
            senses.push(sense);
        }
        senses
    }

    fn definition(&self) -> () {
        todo!();
    }
}
