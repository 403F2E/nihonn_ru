use std::io::{stdin, stdout, Write};
use thiserror::Error;

#[derive(Error, Debug)]
enum ErrorApp {
    #[error("What is this command, I never heard of it 🙃")]
    ErrorCommand,
}

const HELP: [&str; 3] = ["help", "h", "?"];
const PLAY: [&str; 2] = ["play", "p"];
const DEFINITION: [&str; 2] = ["definition", "d"];
const SYNONYME: [&str; 2] = ["synonyme", "s"];

fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        print!("Enter command > ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("There is an error have happened while receiving user input");
        match handle_command(&input.trim()) {
            Ok(()) => continue,
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}

fn handle_command(command: &str) -> Result<(), ErrorApp> {
    match command {
        help if HELP.contains(&help) => Ok(()),
        play if PLAY.contains(&play) => Ok(()),
        definition if DEFINITION.contains(&definition) => Ok(()),
        synom if SYNONYME.contains(&synom) => Ok(()),
        _ => Err(ErrorApp::ErrorCommand),
    }
}
