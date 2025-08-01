use natural_tts::NaturalTts;

use crate::{
    constants::USE_PLAY,
    jisho::{JishoResponse, LogApp},
};

pub fn speak<'a>(
    command: Vec<&str>,
    response: &JishoResponse,
    mut natural: NaturalTts,
) -> Result<(), LogApp<'a>> {
    match command[1].parse::<usize>() {
        Ok(num) => {
            if num > response.data.len() {
                return Err(LogApp::CommandError(
                    "The number given is greater than the words found for your search",
                ));
            }

            let word = &response.data[num - 1].slug;

            if let Err(e) = natural.say_auto(word.to_owned()) {
                return Err(LogApp::ErrorSpeak(e));
            }
        }

        Err(_) => return Err(LogApp::CommandError(USE_PLAY)),
    }

    if let Err(e) = natural.say_auto(command[1].to_owned()) {
        return Err(LogApp::ErrorSpeak(e));
    }

    Ok(())
}
