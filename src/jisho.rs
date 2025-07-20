#![allow(unused)]

use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorApp<'a> {
    #[error("I wish u learned somthing 🙂")]
    GoodBye,
    #[error("An error occured when reaching to the api")]
    ErrorApi,
    #[error("Command Error: {0}")]
    ErrorCommand(&'a str),
    #[error("An error has occured when trying to peak")]
    ErrorSpeak,
}

// all the available commands
pub const QUIT: [&str; 2] = ["exit", "e"];
pub const HELP: [&str; 3] = ["help", "h", "?"];
pub const WORD: [&str; 2] = ["word", "w"];
pub const PLAY: [&str; 2] = ["play", "p"];
pub const EXPLAIN: [&str; 2] = ["explain", "x"];
pub const READING: [&str; 2] = ["read", "r"];
pub const DEFINITION: [&str; 2] = ["definition", "d"];

//pub struct Command<'a> {
//    command: &'a str,
//    arg: Vec<&'a str>,
//}

// Structers to parse the response into
#[derive(Debug, Deserialize)]
pub struct JishoResponse {
    meta: Meta,
    pub data: Vec<WordEntry>,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    status: u32,
}

#[derive(Debug, Deserialize)]
pub struct WordEntry {
    pub slug: String,
    is_common: bool,
    pub japanese: Vec<Japanese>,
    pub senses: Vec<Sense>,
}

#[derive(Debug, Deserialize)]
pub struct Japanese {
    pub word: Option<String>,
    pub reading: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Sense {
    pub english_definitions: Vec<String>,
    pub parts_of_speech: Vec<String>,
}

impl JishoResponse {
    pub fn new(meta: Meta, data: Vec<WordEntry>) -> Self {
        Self { meta, data }
    }
}

impl Meta {
    pub fn new(status: u32) -> Self {
        Self { status }
    }
}

impl WordEntry {
    pub fn word(&self) -> () {
        println!("Word: {}", self.slug);
    }

    pub fn reading(&self, index: usize) {
        println!("Readings: ");
        println!("\t{:?}", self.japanese[index].reading);
    }

    pub fn definition(&self, index: usize) {
        println!("Definition {}:", 1);
        println!(
            "\t{}, {}",
            self.senses[index].parts_of_speech[0], self.senses[index].english_definitions[0]
        )
    }
}
