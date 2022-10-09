pub trait ShadowPKM {
    fn get_shadow_id(&self) -> u16;
    fn set_shadow_id(&mut self, id: u16);

    fn get_purification(&self) -> i32;
    fn set_purification(&mut self, purification: i32);

    fn is_shadow(&self) -> bool;
}
