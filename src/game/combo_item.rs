#[derive(Clone)]
pub struct ComboItem<T: AsRef<str>> {
    pub text: T,
    pub value: i32,
}

impl<T: AsRef<str>> ComboItem<T> {
    pub const fn new(text: T, value: i32) -> ComboItem<T> {
        Self { text, value }
    }
}
