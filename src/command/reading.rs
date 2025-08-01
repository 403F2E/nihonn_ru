use crate::{
    constants::USE_READING,
    jisho::{JishoResponse, LogApp},
};

pub fn reads<'a>(
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
                    word.get_all_readings();
                }
            }
            nums => {
                let nums: Vec<&str> = nums.split(",").collect();
                for word in &response.data {
                    word.word();
                    for num in &nums {
                        match num.parse::<usize>() {
                            Ok(num) => {
                                word.japanese[num].get_reading(num - 1);
                            }
                            Err(_) => {
                                return Err(LogApp::CommandError(
                                    "The 'reads' command accept only numbers or 'all' as argument.",
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
                                word.japanese[num - 1].get_reading(num - 1);
                            }
                            Err(_) => return Err(LogApp::CommandError(USE_READING)),
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
                                    response.data[num - 1].get_all_readings();
                                }
                                Err(_) => {
                                    return Err(LogApp::CommandError(
                                        "The 'reads' command accept only numbers or 'all' as arguments.",
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
                                                response.data[num - 1].japanese[few - 1]
                                                    .get_reading(few - 1);
                                            }
                                            Err(_) => {
                                                return Err(LogApp::CommandError(USE_READING));
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    return Err(LogApp::CommandError(
                                        "The 'reads' command accept only numbers or 'all' as arguments.",
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
