use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]

pub(crate) struct Genre {
    song_count: u32,
    album_count: u32,
}
