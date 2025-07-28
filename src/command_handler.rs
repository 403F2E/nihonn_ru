use std::process::Command;

use crate::{
    command::{definition::define, explain::explains, play::speak, reading::reads, word::search},
    jisho::JishoResponse,
    prelude::*,
};
use natural_tts::NaturalTts;

// Help Menu
fn help_menu() {
    println!("\n-------------------\n HELP MENU \n-------------------");
    println!("To search a word in the dictonary. WORD: word to search in the dictionary: ");
    println!("\t{}", USE_WORD);
    println!("To show all the information about the word: ");
    println!("\t{}", USE_EXPLAIN);
    println!("To play how a word sounds. NUMBER: number of the word: ");
    println!("\t{}", USE_PLAY);
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

// Function that handle the Commands
pub fn handle_command<'a, 'b>(
    command: &'a str,
    response: JishoResponse,
    natural: NaturalTts,
) -> Result<JishoResponse, LogApp<'a>> {
    let command: Vec<&str> = command.split(" ").collect();
    let length: usize = command.len();

    match command[0] {
        /* *
         * WORD: Receive the user word and reach the api for data
         * */
        word if WORD.contains(&word) => {
            if length < 2 || length > 2 {
                return Err(LogApp::ErrorCommand(
                    "The 'word' command should use one argument.",
                ));
            }

            search(command)
        }

        /* *
         * HELP : Print the help menu
         * */
        help if HELP.contains(&help) => {
            if length > 1 {
                return Err(LogApp::ErrorCommand(
                    "The 'help' command does not have any argument.",
                ));
            }

            help_menu();
            Ok(response)
        }

        /* *
         * PLAY : Play the word sounding
         * */
        play if PLAY.contains(&play) => {
            if length < 2 || length > 2 {
                return Err(LogApp::ErrorCommand(
                    "you can only play one word at the time.",
                ));
            }

            let (is_numeric, is_alphanumeric) = (
                command[1].chars().all(|c| c.is_numeric()),
                command[1].chars().all(|c| c.is_alphanumeric()),
            );
            if !is_numeric && !is_alphanumeric {
                return Err(LogApp::ErrorCommand(USE_PLAY));
            }

            if response.data.len() == 0 {
                return Err(LogApp::ErrorCommand("The dictonary is empty."));
            }

            let _ = speak(command, &response, natural)?;
            Ok(response)
        }

        /* *
         * EXPLAIN : Print all the information fetched about the searched word
         * */
        explain if EXPLAIN.contains(&explain) => {
            let _ = explains(command, &response)?;
            Ok(response)
        }

        /* *
         * DEFINITION : Print the definitions or definition of the word
         * */
        definition if DEFINITION.contains(&definition) => {
            if length < 2 || length > 3 {
                return Err(LogApp::ErrorCommand(
                    "the 'define' command should have one argument",
                ));
            }

            if !command[1]
                .chars()
                .all(|c| c.is_ascii_punctuation() || c.is_ascii_alphanumeric())
            {
                return Err(LogApp::ErrorCommand(USE_EXPLAIN));
            }

            if response.data.len() == 0 {
                return Err(LogApp::ErrorCommand("The dictonary is empty!"));
            }

            let _ = define(command, length, &response)?;
            Ok(response)
        }

        /* *
         * READING : Print the readings or reading of the word
         * */
        reading if READING.contains(&reading) => {
            if length < 2 || length > 3 {
                return Err(LogApp::ErrorCommand(
                    "the 'reads' command should have one argument",
                ));
            }

            if !command[1]
                .chars()
                .all(|c| c.is_ascii_punctuation() || c.is_ascii_alphanumeric())
            {
                return Err(LogApp::ErrorCommand(USE_EXPLAIN));
            }

            if response.data.len() == 0 {
                return Err(LogApp::ErrorCommand("The dictonary is empty!"));
            }

            let _ = reads(command, length, &response)?;
            Ok(response)
        }

        /* *
         * CLEAR : Clear the terminal
         * */
        clear if CLEAR.contains(&clear) => {
            if length > 1 {
                return Err(LogApp::ErrorCommand(
                    "The 'clear' command takes no arguments.",
                ));
            }

            if cfg!(target_os = "windows") {
                let _ = Command::new("cmd").args(["/c", "cls"]).spawn().unwrap();
            } else {
                let _ = Command::new("clear").status().unwrap();
            }

            Ok(response)
        }

        /* *
         * EXIT : Quit the program
         * */
        quit if QUIT.contains(&quit) => Err(LogApp::GoodBye),

        /* *
         * UNKNOWN COMMAND : In case of Unknown Commands
         * */
        _ => Err(LogApp::ErrorCommand(
            "What is this command, I never heard of it 🙃",
        )),
    }
}
