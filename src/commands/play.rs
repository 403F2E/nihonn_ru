use natural_tts::NaturalTts;

use crate::{
    constants::USE_PLAY,
    jisho::{AppLog, JishoResponse},
};

pub fn speak<'a>(
    command: Vec<&str>,
    response: &JishoResponse,
    mut natural: NaturalTts,
) -> Result<(), AppLog<'a>> {
    match command[1].parse::<usize>() {
        Ok(num) => {
            if num > response.data.len() {
                return Err(AppLog::CommandError(
                    "The number given is greater than the words found for your search",
                ));
            }

            let word = &response.data[num - 1].slug;

            if let Err(e) = natural.say_auto(word.to_owned()) {
                return Err(AppLog::ErrorSpeak(e));
            }
        }

        Err(_) => return Err(AppLog::CommandInfo(USE_PLAY)),
    }

    if let Err(e) = natural.say_auto(command[1].to_owned()) {
        return Err(AppLog::ErrorSpeak(e));
    }

    Ok(())
}
