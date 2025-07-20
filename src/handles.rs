use std::error::Error;

use crate::jisho::*;
use natural_tts::{
    models::gtts::{languages, GttsModel},
    *,
};
use reqwest;

#[tokio::main]
pub async fn api_handle(keyword: &str) -> Result<JishoResponse, ErrorApp> {
    let full_url: String = format!("https://jisho.org/api/v1/search/words?keyword={}", keyword);
    let resp: JishoResponse = match reqwest::get(&full_url)
        .await
        .unwrap()
        .json::<JishoResponse>()
        .await
    {
        Ok(resp) => resp,
        Err(_) => {
            return Err(ErrorApp::ErrorApi);
        }
    };
    Ok(resp)
}

pub fn setup_handle() -> Result<(JishoResponse, NaturalTts), Box<dyn Error>> {
    // Building the text-to-speech struct
    let natural: NaturalTts = NaturalTtsBuilder::default()
        .gtts_model(GttsModel::new(
            0.8,
            languages::Languages::Japanese,
            "com".to_owned(),
        ))
        .default_model(Model::Gtts)
        .build()?;

    let resp: JishoResponse = JishoResponse::new(Meta::new(0), Vec::new());

    Ok((resp, natural))
}

pub fn help_menu() {
    println!("\n-------------------\n HELP MENU \n-------------------");
    println!("word [or w] (WORD)");
    println!("\tTo search a word in the dictonary. WORD: word to search in the dictionary");
    println!("explain [or x]");
    println!("\tTo show all the information about the word.");
    println!(
        "play [or p] -r (NUMBER) (NUMBER/all). [-w NUMBER: word's number to show reading of, NUMBER: definitions number OR all: show all the readings]"
    );
    println!("\tTo play how a word sounds. NUMBER: number of the word");
    println!("definition [or d] -w (NUMBER) (NUMBER/all)");
    println!(
        "\tTo show definition of the word. [-w NUMBER: word's number to show definitions of, NUMBER: definitions number OR all: show all the definitions]"
    );
    println!("reading [or r] -w (NUMBER) (NUMBER/all)");
    println!("\tTo show reading of a word. [-w NUMBER: ]");
    println!("help [or h]");
    println!("\tDisplay this help menu.");
    println!("quit [or q]");
    println!("\tcommand to search a word in the dictonary\n");
}

// TODO: handle showing information
#[allow(dead_code)]
fn structure_handle(data: Vec<WordEntry>) -> () {
    for word in data {
        word.word();
        word.reading(word.japanese.len());
        word.definition(word.senses.len());
    }
}
