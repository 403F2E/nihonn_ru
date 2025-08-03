mod api_request;
mod command_handler;
mod commands;
mod constants;
mod jisho;
mod setup;

use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use command_handler::handle_command;
use jisho::{AppLog, JishoResponse};
use setup::setup_handle;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut response, natural): (JishoResponse, natural_tts::NaturalTts) = setup_handle()?;

    /* *
     * Receiving user's command
     * */
    let mut input = String::new();
    loop {
        input.clear();
        print!("Enter command > ");
        stdout().flush()?;
        stdin()
            .read_line(&mut input)
            .expect("There is an error have happened while receiving user input");
        match handle_command(&input.trim(), response.clone(), natural.clone()) {
            Ok(response_state) => {
                response = response_state;
                continue;
            }
            Err(e) => {
                println!("{}", e);
                match e {
                    AppLog::GoodBye => {
                        break;
                    }
                    _ => {
                        continue;
                    }
                }
            }
        };
    }
    Ok(())
}
