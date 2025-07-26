use reqwest;

use crate::{jisho::JishoResponse, prelude::ErrorApp};

#[tokio::main]
pub async fn api_handle(keyword: &str) -> Result<JishoResponse, ErrorApp> {
    let full_url: String = format!("https://jisho.org/api/v1/search/words?keyword={}", keyword);
    let response: JishoResponse = match reqwest::get(&full_url)
        .await
        .unwrap()
        .json::<JishoResponse>()
        .await
    {
        Ok(resp) => resp,
        Err(_) => {
            return Err(ErrorApp::ErrorApi);
        }
    };
    Ok(response)
}
