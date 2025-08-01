use crate::{
    constants::USE_EXPLAIN,
    jisho::{JishoResponse, LogApp},
};

pub fn explains<'a>(command: Vec<&str>, response: &JishoResponse) -> Result<(), LogApp<'a>> {
    let length: usize = command.len();
    if length < 2 || length > 2 {
        return Err(LogApp::CommandError(
            "the 'explain' command should have one argument",
        ));
    }

    if !command[1]
        .chars()
        .all(|c| c.is_ascii_punctuation() || c.is_ascii_alphanumeric())
    {
        return Err(LogApp::CommandError(USE_EXPLAIN));
    }

    if response.data.len() == 0 {
        return Err(LogApp::CommandInfo("The dictionary is empty!"));
    }

    match command[1] {
        "all" => {
            for word in &response.data {
                word.word();
                word.get_all_readings();
                word.define_all();
            }
        }
        "help" => return Err(LogApp::CommandError(USE_EXPLAIN)),
        nums => {
            let nums: Vec<&str> = nums.split(",").collect();
            println!("{nums:?}");
            for num in nums {
                match num.parse::<usize>() {
                    Ok(num) => {
                        if num >= length {
                            return Err(LogApp::CommandInfo(
                                "The dictionary only have {length:?}. {num:?} not valid.",
                            ));
                        }
                        response.data[num - 1].word();
                        response.data[num - 1].get_all_readings();
                        response.data[num - 1].define_all();
                    }
                    Err(_) => {
                        return Err(LogApp::CommandError(
                            "The 'explain' command accept only numbers or all as argument.",
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}
