#![allow(dead_code)]

/// * *
/// * used types overall the projects
/// * *
use serde::Deserialize;
use thiserror::Error;

use crate::constants::{HIRAGANA_ROMANJI, KATAKANA_ROMANJI};

// Error type to handle types
#[derive(Error, Debug)]
pub enum AppLog<'a> {
    #[error("I wish u learned somthing 🙂")]
    GoodBye,
    #[error("An error occured when reaching to the api")]
    ErrorApi,
    #[error("{0}")]
    CommandInfo(&'a str),
    #[error("Command Error: {0}")]
    CommandError(&'a str),
    #[error("An error has occured when trying to peak: \n{0}")]
    ErrorSpeak(Box<dyn std::error::Error>),
}

// Structers to parse the api response into
#[derive(Debug, Deserialize, Clone)]
pub struct JishoResponse {
    meta: Meta,
    pub data: Vec<WordEntry>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Meta {
    status: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WordEntry {
    pub slug: String,
    is_common: bool,
    pub japanese: Vec<Japanese>,
    pub senses: Vec<Sense>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Japanese {
    pub word: Option<String>,
    pub reading: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
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
        println!();
        println!("Word: {}", self.slug);
    }

    pub fn get_all_readings(&self) {
        println!("Readings : ");
        for index in 0..self.japanese.len() {
            self.japanese[index].get_reading(index);
        }
    }

    pub fn define_all(&self) {
        for index in 0..self.senses.len() {
            self.senses[index].define_one(index);
        }
        println!();
    }
}

impl Japanese {
    pub fn get_reading(&self, index: usize) {
        let reading = self.reading.clone().unwrap();
        println!("\tReading {} : ", index + 1);
        print!("\t\t- {} (", reading);
        for c in reading.chars() {
            let romanji = HIRAGANA_ROMANJI
                .get(&c)
                .or_else(|| KATAKANA_ROMANJI.get(&c))
                .unwrap_or(&"?");
            print!("{}", romanji);
        }
        print!(")\n");
    }
}

impl Sense {
    pub fn define_one(&self, index: usize) {
        println!("Definition {}:", index + 1);
        println!(
            "\t{}, {}",
            self.parts_of_speech[0], self.english_definitions[0]
        )
    }
}
