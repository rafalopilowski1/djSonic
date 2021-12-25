

use serde::Deserialize;

use crate::data_structure::child::Child;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Directory {
    #[serde(rename="$value")]
    childes: Vec<Child>,
    id: String,
    parent: Option<String>,
    name: String,
    starred: Option<String>,
    user_rating: Option<u32>,    // UserRating: [1,5]
    average_rating: Option<f32>, // AverageRating: [1.0,5.0]
    play_count: Option<u64>,
}
