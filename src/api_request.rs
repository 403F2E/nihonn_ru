use reqwest;

use crate::jisho::{AppLog, JishoResponse};

#[tokio::main]
pub async fn api_handle(keyword: &str) -> Result<JishoResponse, AppLog> {
    let full_url: String = format!("https://jisho.org/api/v1/search/words?keyword={}", keyword);
    let response: JishoResponse = match reqwest::get(&full_url)
        .await
        .unwrap()
        .json::<JishoResponse>()
        .await
    {
        Ok(resp) => resp,
        Err(_) => {
            return Err(AppLog::ErrorApi);
        }
    };
    Ok(response)
}
