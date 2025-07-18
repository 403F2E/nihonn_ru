use crate::jisho::*;
use reqwest;
use std::collections::HashMap;

//const API_URL: &str = ;

#[tokio::main]
pub async fn show_response() -> Result<(), Box<dyn std::error::Error>> {
    let full_url = format!("https://jisho.org/api/v1/search/words?keyword={}", "apple");
    let resp = reqwest::get(&full_url)
        .await?
        .json::<JishoResponse>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}
