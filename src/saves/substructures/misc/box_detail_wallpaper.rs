pub trait BoxDetailWallpaper {
    fn get_box_wallpaper(&self, index: u8);
    fn set_box_wallpaper(&self, index: u8, value: u8);
}
