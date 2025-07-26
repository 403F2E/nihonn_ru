use std::process::Command;

use crate::{api_request::api_handle, jisho::JishoResponse, prelude::*};
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
    mut natural: NaturalTts,
) -> Result<JishoResponse, LogApp<'a>> {
    let command: Vec<&str> = command.split(" ").collect();
    let length: usize = command.len();

    match command[0] {
        /* *
         * Receive the user word and reach the api for data
         * */
        word if WORD.contains(&word) => {
            if length < 2 || length > 2 {
                return Err(LogApp::ErrorCommand(
                    "The 'word' command should use one argument.",
                ));
            }

            if command[1].chars().all(|c| c.is_alphanumeric()) {
                return api_handle(command[1]);
            }

            Err(LogApp::ErrorCommand(
                "The word that you are searching should be alphanumerical.",
            ))
        }

        /* *
         * Print the help menu
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
         * Play the word sounding
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

            match command[1].parse::<usize>() {
                Ok(num) => {
                    if num > response.data.len() {
                        return Err(LogApp::ErrorCommand(
                            "The number given is greater than the words found for your search",
                        ));
                    }

                    let word = &response.data[num - 1].slug;
                    if let Err(e) = natural.say_auto(word.to_owned()) {
                        return Err(LogApp::ErrorSpeak(e));
                    }
                }
                Err(_) => {
                    return Err(LogApp::ErrorCommand(USE_PLAY));
                }
            }

            if let Err(e) = natural.say_auto(command[1].to_owned()) {
                return Err(LogApp::ErrorSpeak(e));
            }

            Ok(response)
        }

        // Print all the information fetched about the searched word
        explain if EXPLAIN.contains(&explain) => {
            if length < 2 || length > 2 {
                return Err(LogApp::ErrorCommand(
                    "the 'explain' command should have one argument",
                ));
            }

            if !command[1].chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err(LogApp::ErrorCommand(USE_EXPLAIN));
            }

            if response.data.len() == 0 {
                return Err(LogApp::ErrorCommand("The dictonary is empty!"));
            }

            match command[1] {
                "all" => {
                    for word in &response.data {
                        word.word();
                        word.readings();
                        word.definitions();
                    }
                }
                _ => {
                    let nums: Vec<&str> = command[1].split(",").collect();
                    for num in nums {
                        match num.parse::<usize>() {
                            Ok(num) => {
                                response.data[num].word();
                                response.data[num].readings();
                                response.data[num].definitions();
                            }
                            Err(_) => {
                                return Err(LogApp::ErrorCommand(
                                    "The 'explain' command accept only numbers or all as argument.",
                                ));
                            }
                        }
                    }
                }
            }
            Ok(response)
        }

        // Print the definitions or definition of the word
        definition if DEFINITION.contains(&definition) => Ok(response),

        // Print the readings or reading of the word
        reading if READING.contains(&reading) => Ok(response),

        // Clear the terminal
        clear if CLEAR.contains(&clear) => {
            if length > 1 {
                return Err(LogApp::ErrorCommand(
                    "The 'clear' command takes no arguments.",
                ));
            }

            if cfg!(target_os = "windows") {
                let _ = Command::new("cmd").args(["/c", "cls"]).spawn()?;
            } else {
                let _ = Command::new("clear").status()?;
            }

            Ok(response)
        }

        // Quit the program
        quit if QUIT.contains(&quit) => Err(LogApp::GoodBye),

        // In case of Unknown Commands
        _ => Err(LogApp::ErrorCommand(
            "What is this command, I never heard of it 🙃",
        )),
    }
}
