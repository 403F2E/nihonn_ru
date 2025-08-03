use crate::{
    constants::USE_EXPLAIN,
    jisho::{AppLog, JishoResponse},
};

pub fn explains<'a>(command: Vec<&str>, response: &JishoResponse) -> Result<(), AppLog<'a>> {
    let length: usize = command.len();
    if length < 2 || length > 2 {
        return Err(AppLog::CommandError(
            "the 'explain' command should have one argument",
        ));
    }

    if !command[1]
        .chars()
        .all(|c| c.is_ascii_punctuation() || c.is_ascii_alphanumeric())
    {
        return Err(AppLog::CommandInfo(USE_EXPLAIN));
    }

    if response.data.len() == 0 {
        return Err(AppLog::CommandInfo("The dictionary is empty!"));
    }

    match command[1] {
        "help" => return Err(AppLog::CommandInfo(USE_EXPLAIN)),
        "all" => {
            for word in &response.data {
                word.word();
                word.get_all_readings();
                word.define_all();
            }
        }
        nums => {
            let nums: Vec<&str> = nums.split(",").collect();
            for num in nums {
                match num.parse::<usize>() {
                    Ok(num) => {
                        if num >= length {
                            return Err(AppLog::CommandInfo(
                                "The dictionary only have {length:?}. {num:?} not valid.",
                            ));
                        }
                        response.data[num - 1].word();
                        response.data[num - 1].get_all_readings();
                        response.data[num - 1].define_all();
                    }
                    Err(_) => {
                        return Err(AppLog::CommandError(
                            "The 'explain' command accept only numbers or all as argument.",
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}
