use std::error::Error;

use crate::jisho::{JishoResponse, Meta};
use natural_tts::{
    models::gtts::{languages, GttsModel},
    *,
};

pub fn setup_handle() -> Result<(JishoResponse, NaturalTts), Box<dyn Error>> {
    // Building the text-to-speech struct
    let natural: NaturalTts = NaturalTtsBuilder::default()
        .gtts_model(GttsModel::new(
            0.8,
            languages::Languages::Japanese,
            "com".to_owned(),
        ))
        .default_model(Model::Gtts)
        .build()?;

    let resp: JishoResponse = JishoResponse::new(Meta::new(0), Vec::new());

    Ok((resp, natural))
}
