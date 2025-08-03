use crate::{
    api_request::api_handle,
    jisho::{AppLog, JishoResponse},
};

pub fn search(command: Vec<&str>) -> Result<JishoResponse, AppLog<'_>> {
    if command[1].chars().all(|c| c.is_alphanumeric()) {
        return api_handle(command[1]);
    }

    Err(AppLog::CommandError(
        "The word that you are searching should be alphanumerical.",
    ))
}
