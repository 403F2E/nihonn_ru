mod handles;
mod jisho;

use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use handles::{api_handle, help_menu, setup_handle};
use jisho::*;
use natural_tts::NaturalTts;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut resp, natural) = setup_handle()?;

    // Accepting users command
    let mut input = String::new();
    loop {
        input.clear();
        print!("Enter command > ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("There is an error have happened while receiving user input");
        match handle_command(&input.trim(), resp, natural.clone()) {
            Ok(resp_state) => {
                resp = resp_state;
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
fn handle_command(
    command: &str,
    resp: JishoResponse,
    mut natural: NaturalTts,
) -> Result<JishoResponse, ErrorApp> {
    let command: Vec<&str> = command.split(" ").collect();

    match command[0] {
        // Receive the user word and reach the api for data
        word if WORD.contains(&word) => match api_handle(command[1]) {
            Ok(resp) => Ok(resp),
            Err(e) => Err(e),
        },

        // Print the help menu
        help if HELP.contains(&help) => {
            help_menu();
            Ok(resp)
        }

        // Play the word sounding
        play if PLAY.contains(&play) => match command[1].parse::<usize>() {
            Ok(num) => {
                if num < resp.data.len() {
                    let word = &resp.data[num - 1].slug;
                    let _ = natural.say_auto(word.to_owned());
                    Ok(resp)
                } else {
                    Err(ErrorApp::ErrorCommand(
                        "Number high than the existant words.",
                    ))
                }
            }
            Err(_) => Err(ErrorApp::ErrorCommand(
                "The correct form is : play [or p] (THE NUMBER OF THE WANTED WORD)",
            )),
        },

        // Print the definitions or definition of the word
        definition if DEFINITION.contains(&definition) => Ok(resp),

        // Quit the program
        quit if QUIT.contains(&quit) => Err(ErrorApp::GoodBye),

        // In case of Unknown Commands
        _ => Err(ErrorApp::ErrorCommand(
            "What is this command, I never heard of it 🙃",
        )),
    }
}
