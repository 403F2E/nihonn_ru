#![allow(dead_code)]

use serde::Deserialize;

// Structers to parse the response into
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

    pub fn reading(&self, index: usize) {
        println!("Reading: ");
        println!("\t{:?}", self.japanese[index].reading);
    }

    pub fn readings(&self) {
        for index in 0..self.japanese.len() {
            println!("Reading {}: ", index);
            println!("\t{:?}", self.japanese[index].reading);
        }
    }

    pub fn word_definitions(&self) {
        for index in 0..self.senses.len() {
            self.senses[index].definition(index);
        }
        println!();
    }
}

impl Japanese {
    pub fn reading(&self, index: usize) {
        let reading = self.reading.clone();
        println!("Reading {} : ", index);
        println!("\t-{}", reading.unwrap());
    }
}

impl Sense {
    pub fn definition(&self, index: usize) {
        println!("Definition {}:", index);
        println!(
            "\t{}, {}",
            self.parts_of_speech[0], self.english_definitions[0]
        )
    }
}
