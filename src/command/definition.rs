use crate::{
    jisho::JishoResponse,
    prelude::{LogApp, USE_DEFINITION},
};

pub fn definition<'a>(
    command: Vec<&str>,
    length: usize,
    response: &JishoResponse,
) -> Result<(), LogApp<'a>> {
    // in case of single argument
    if length == 2 {
        match command[1] {
            "all" => {
                for word in &response.data {
                    println!("for word : {} ", word.slug);
                    word.word_definitions();
                }
            }
            nums => {
                let nums: Vec<&str> = nums.split(",").collect();
                for word in &response.data {
                    word.word();
                    for num in &nums {
                        match num.parse::<usize>() {
                            Ok(num) => {
                                word.senses[num].definition(num);
                            }
                            Err(_) => {
                                return Err(LogApp::ErrorCommand(
                                    "The 'definition' command accept only numbers or 'all' as argument.",
                                ));
                            }
                        }
                    }
                }
            }
        }
    } else {
        // in case of 2 arguments
        match command[2] {
            "all" => {
                // if first argument = 1,2,...
                let index: Vec<&str> = command[1].split(",").collect();
                for word in &response.data {
                    word.word();
                    for num in index.iter() {
                        match num.parse::<usize>() {
                            Ok(num) => {
                                word.senses[num].definition(num);
                            }
                            Err(_) => return Err(LogApp::ErrorCommand(USE_DEFINITION)),
                        }
                    }
                }
            }
            nums => {
                let nums: Vec<&str> = nums.split(",").collect();
                match command[1] {
                    "all" => {
                        for num in nums {
                            match num.parse::<usize>() {
                                Ok(num) => {
                                    response.data[num].word_definitions();
                                }
                                Err(_) => {
                                    return Err(LogApp::ErrorCommand(
                                        "The 'definition' command accept only numbers or 'all' as arguments.",
                                    ));
                                }
                            }
                        }
                    }
                    fews => {
                        let fews: Vec<&str> = fews.split(",").collect();
                        for num in nums {
                            match num.parse::<usize>() {
                                Ok(num) => {
                                    response.data[num].word();
                                    for few in &fews {
                                        match few.parse::<usize>() {
                                            Ok(few) => {
                                                response.data[num].senses[few].definition(few);
                                            }
                                            Err(_) => {
                                                return Err(LogApp::ErrorCommand(USE_DEFINITION));
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    return Err(LogApp::ErrorCommand(
                                        "The 'definition' command accept only numbers or 'all' as arguments.",
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
