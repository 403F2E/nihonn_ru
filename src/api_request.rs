use reqwest;

use crate::jisho::{JishoResponse, LogApp};

#[tokio::main]
pub async fn api_handle(keyword: &str) -> Result<JishoResponse, LogApp> {
    let full_url: String = format!("https://jisho.org/api/v1/search/words?keyword={}", keyword);
    let response: JishoResponse = match reqwest::get(&full_url)
        .await
        .unwrap()
        .json::<JishoResponse>()
        .await
    {
        Ok(resp) => resp,
        Err(_) => {
            return Err(LogApp::ErrorApi);
        }
    };
    Ok(response)
}
