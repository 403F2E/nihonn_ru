use crate::{api_request::api_handle, jisho::JishoResponse, prelude::LogApp};

pub fn word(command: Vec<&str>) -> Result<JishoResponse, LogApp<'_>> {
    if command[1].chars().all(|c| c.is_alphanumeric()) {
        return api_handle(command[1]);
    }

    Err(LogApp::ErrorCommand(
        "The word that you are searching should be alphanumerical.",
    ))
}
