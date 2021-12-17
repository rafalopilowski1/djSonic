use std::time::Duration;

use crate::data_structure::child::Child;

pub(crate) struct Directory {
    childes: Vec<Child>,
    id: String,
    parent: Option<String>,
    name: String,
    starred: Option<Duration>,
    user_rating: Option<u32>,    // UserRating: [1,5]
    average_rating: Option<f32>, // AverageRating: [1.0,5.0]
    play_count: Option<u64>,
}
