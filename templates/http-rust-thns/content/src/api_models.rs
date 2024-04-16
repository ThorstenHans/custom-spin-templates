use serde::Serialize;
use spin_sdk::http::conversions::IntoBody;

#[derive(Serialize)]
pub(crate) struct HelloYouResponseModel {
    pub(crate) greeting: String,
    pub(crate) name: String,
    #[serde(rename = "composedGreeting")]
    pub(crate) composed_greeting: String,
}

impl IntoBody for HelloYouResponseModel {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}
