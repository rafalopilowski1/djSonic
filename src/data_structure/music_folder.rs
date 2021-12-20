use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MusicFolder {
    id: u32,
    name: Option<String>,
}
