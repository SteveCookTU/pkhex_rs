#[derive(Clone)]
pub struct ComboItem<T: AsRef<str> + Clone> {
    pub text: T,
    pub value: i32,
}
