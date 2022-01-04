pub(crate) trait CoverArt {
    fn get_cover_art_id(&self) -> Option<&str>;
}
pub(crate) trait Streamable {
    fn get_id(&self) -> &str;
}
