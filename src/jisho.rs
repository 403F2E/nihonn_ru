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
        let reading = self.reading.clone();
        println!("\tReading {} : ", index + 1);
        println!("\t\t-{}", reading.unwrap());
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
