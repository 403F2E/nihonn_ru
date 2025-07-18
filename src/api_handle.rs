use crate::jisho::*;
use reqwest;

#[tokio::main]
pub async fn get_response(keyword: &str) -> Result<JishoResponse, ErrorApp> {
    let full_url = format!("https://jisho.org/api/v1/search/words?keyword={}", keyword);
    let resp = match reqwest::get(&full_url)
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
    Ok(resp)
}
