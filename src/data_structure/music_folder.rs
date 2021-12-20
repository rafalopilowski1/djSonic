use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MusicFolder {
    id: u32,
    name: Option<String>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MusicFolders {
    #[serde(rename="$value")]
    values: Vec<MusicFolder>
}