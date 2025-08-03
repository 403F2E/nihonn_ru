use crate::{
    constants::USE_DEFINITION,
    jisho::{AppLog, JishoResponse},
};

pub fn define<'a>(
    command: Vec<&str>,
    length: usize,
    response: &JishoResponse,
) -> Result<(), AppLog<'a>> {
    // in case of single argument
    if length == 2 {
        match command[1] {
            "help" => return Err(AppLog::CommandInfo(USE_DEFINITION)),
            "all" => {
                for word in &response.data {
                    println!("for word : {} ", word.slug);
                    word.define_all();
                }
            }
            nums => {
                let nums: Vec<&str> = nums.split(",").collect();
                for word in &response.data {
                    word.word();
                    for num in &nums {
                        match num.parse::<usize>() {
                            Ok(num) => {
                                word.senses[num - 1].define_one(num - 1);
                            }
                            Err(_) => {
                                return Err(AppLog::CommandError(
                                    "The 'define' command accept only numbers or 'all' as argument.",
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
                                word.senses[num - 1].define_one(num - 1);
                            }
                            Err(_) => return Err(AppLog::CommandInfo(USE_DEFINITION)),
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
                                    response.data[num - 1].define_all();
                                }
                                Err(_) => {
                                    return Err(AppLog::CommandError(
                                        "The 'define' command accept only numbers or 'all' as arguments.",
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
                                                response.data[num - 1].senses[few - 1]
                                                    .define_one(few - 1);
                                            }
                                            Err(_) => {
                                                return Err(AppLog::CommandInfo(USE_DEFINITION));
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    return Err(AppLog::CommandError(
                                        "The 'define' command accept only numbers or 'all' as arguments.",
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
