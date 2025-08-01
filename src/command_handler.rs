use std::process::Command;

use crate::{
    command::{definition::define, explain::explains, play::speak, reading::reads, word::search},
    constants::*,
    jisho::{JishoResponse, LogApp},
};
use natural_tts::NaturalTts;

// Help Menu
fn help_menu() {
    println!("\n-------------------------\n \tHELP MENU \n-------------------------\n");
    println!("To search a word in the dictonary: ");
    println!("\t> search apple \t# ('search help' for more.)\n");
    println!("To show all the information about the word: ");
    println!("\t> explain all \t# ('explain help' for more.)\n");
    println!("To play how a word sounds: ");
    println!("\t> play 林檎 \t# ('play help' for more.)\n");
    println!("To show the word's definitions: ");
    println!("\t> define 1 all \t# ('define help' for more.)\n");
    println!("To show the word's readings: ");
    println!("\t> reads 1 all \t# ('reads help' for more.)\n");
    println!("Display this help menu.");
    println!("\t> help [or h or ?]\n");
    println!("To clear the terminal.");
    println!("\t> clear [or c]\n");
    println!("To exit the program.");
    println!("\t> quit [or q]\n");
}

/// Function that parses and executes the Commands
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
                return Err(LogApp::CommandError(
                    "The 'word' command should use one argument.",
                ));
            }

            match command[1] {
                "help" => return Err(LogApp::CommandInfo(USE_WORD)),
                _ => search(command),
            }
        }

        /* *
         * HELP : Print the help menu
         * */
        help if HELP.contains(&help) => {
            if length > 1 {
                return Err(LogApp::CommandError(
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
                return Err(LogApp::CommandError(
                    "you can only play one word at the time.",
                ));
            }

            let (is_numeric, is_alphanumeric) = (
                command[1].chars().all(|c| c.is_numeric()),
                command[1].chars().all(|c| c.is_alphanumeric()),
            );
            if !is_numeric && !is_alphanumeric {
                return Err(LogApp::CommandError(USE_PLAY));
            }

            if response.data.len() == 0 {
                return Err(LogApp::CommandError("The dictonary is empty."));
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
                return Err(LogApp::CommandError(
                    "the 'define' command should have one argument",
                ));
            }

            if !command[1]
                .chars()
                .all(|c| c.is_ascii_punctuation() || c.is_ascii_alphanumeric())
            {
                return Err(LogApp::CommandError(USE_EXPLAIN));
            }

            if response.data.len() == 0 {
                return Err(LogApp::CommandInfo("The dictonary is empty!"));
            }

            let _ = define(command, length, &response)?;
            Ok(response)
        }

        /* *
         * READING : Print the readings or reading of the word
         * */
        reading if READING.contains(&reading) => {
            if length < 2 || length > 3 {
                return Err(LogApp::CommandError(
                    "the 'reads' command should have one argument",
                ));
            }

            if !command[1]
                .chars()
                .all(|c| c.is_ascii_punctuation() || c.is_ascii_alphanumeric())
            {
                return Err(LogApp::CommandError(USE_EXPLAIN));
            }

            if response.data.len() == 0 {
                return Err(LogApp::CommandInfo("The dictonary is empty!"));
            }

            let _ = reads(command, length, &response)?;
            Ok(response)
        }

        /* *
         * CLEAR : Clear the terminal
         * */
        clear if CLEAR.contains(&clear) => {
            if length > 1 {
                return Err(LogApp::CommandError(
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
        _ => Err(LogApp::CommandInfo(
            "What is this command, I never heard of it 🙃",
        )),
    }
}
