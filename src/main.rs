mod api_handle;
mod jisho;

use natural_tts::{
    models::gtts::{languages, GttsModel},
    *,
};
use std::{
    error::{self, Error},
    io::{stdin, stdout, Write},
    process::exit,
};

use api_handle::get_response;
use jisho::*;

// Function that handles the speaking feature
fn speak(input: String) -> Result<(), ErrorApp> {
    // Building the text-to-speech struct
    let mut natural = NaturalTtsBuilder::default()
        .gtts_model(GttsModel::new(
            0.8,
            languages::Languages::Japanese,
            "com".to_owned(),
        ))
        .default_model(Model::Gtts)
        .build()
        .unwrap();

    if let Err(_) = natural.say_auto(input) {
        return Err(ErrorApp::ErrorSpeak);
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let resp = match get_response("apple") {
        Ok(resp) => resp,
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    };
    println!("{resp:#?}");

    // Accepting users command
    let mut input = String::new();
    loop {
        input.clear();
        print!("Enter command > ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("There is an error have happened while receiving user input");
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

// Function that handle the Commands
fn handle_command(command: &str) -> Result<(), ErrorApp> {
    match command {
        help if HELP.contains(&help) => Ok(()),
        play if PLAY.contains(&play) => speak(command.to_owned()),
        definition if DEFINITION.contains(&definition) => Ok(()),
        synom if SYNONYME.contains(&synom) => Ok(()),
        quit if QUIT.contains(&quit) => Err(ErrorApp::GoodBye),
        _ => Err(ErrorApp::ErrorCommand),
    }
}
