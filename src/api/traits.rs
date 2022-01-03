pub(crate) trait CoverArt {
    fn get_cover_art_id(&self) -> Option<&str>;
}
