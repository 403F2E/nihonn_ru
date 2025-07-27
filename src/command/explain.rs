use crate::{
    jisho::JishoResponse,
    prelude::{LogApp, USE_EXPLAIN},
};

pub fn explain<'a>(command: Vec<&str>, response: &JishoResponse) -> Result<(), LogApp<'a>> {
    let length: usize = command.len();
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
                word.word_definitions();
            }
        }
        _ => {
            let nums: Vec<&str> = command[1].split(",").collect();
            for num in nums {
                match num.parse::<usize>() {
                    Ok(num) => {
                        response.data[num].word();
                        response.data[num].readings();
                        response.data[num].word_definitions();
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
    Ok(())
}
