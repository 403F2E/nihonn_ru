mod api_request;
mod command;
mod jisho;
mod prelude;
mod setup;

use std::{
    error::Error,
    io::{stdin, stdout, Write},
};

use command::handle_command;
use setup::setup_handle;

use crate::prelude::LogApp;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut response, natural): (jisho::JishoResponse, natural_tts::NaturalTts) = setup_handle()?;

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
                    LogApp::GoodBye => {
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
