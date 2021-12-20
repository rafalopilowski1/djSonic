use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub(crate) struct InternetRadioStation {
    id: String,
    name: String,
    stream_url: String,
    home_page_url: Option<String>,
}
