use crate::{
    api_request::api_handle,
    jisho::{JishoResponse, LogApp},
};

pub fn search(command: Vec<&str>) -> Result<JishoResponse, LogApp<'_>> {
    if command[1].chars().all(|c| c.is_alphanumeric()) {
        return api_handle(command[1]);
    }

    Err(LogApp::CommandError(
        "The word that you are searching should be alphanumerical.",
    ))
}
