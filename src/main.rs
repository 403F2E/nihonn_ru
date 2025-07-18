mod jisho;
#[allow(unused)]
mod test_req;

use natural_tts::{
    models::gtts::{languages, GttsModel},
    *,
};
use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use jisho::*;
use test_req::show_response;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = show_response();

    // building the text-to-speech struct
    let mut natural = NaturalTtsBuilder::default()
        .gtts_model(GttsModel::new(
            0.8,
            languages::Languages::Japanese,
            "com".to_owned(),
        ))
        .default_model(Model::Gtts)
        .build()?;

    // accepting users command
    let mut input = String::new();
    loop {
        input.clear();
        print!("Enter command > ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("There is an error have happened while receiving user input");
        let _ = natural.say_auto(input.clone())?;
        match handle_command(&input.trim()) {
            Ok(()) => {
                continue;
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
    Ok(())
}

// function that handle the Commands
fn handle_command(command: &str) -> Result<(), ErrorApp> {
    match command {
        help if HELP.contains(&help) => Ok(()),
        play if PLAY.contains(&play) => Ok(()),
        definition if DEFINITION.contains(&definition) => Ok(()),
        synom if SYNONYME.contains(&synom) => Ok(()),
        _ => Err(ErrorApp::ErrorCommand),
    }
}
