use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Genre {
    song_count: u32,
    album_count: u32,
    #[serde(rename = "$value")]
    name: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Genres {
    #[serde(rename = "$value")]
    values: Vec<Genre>,
}
impl Genres {
    pub(crate) fn getValue(self) -> Vec<Genre> {
        self.values
    }
}
impl Genre {
    pub(crate) fn getName(&self) -> &str {
        &self.name
    }
}
