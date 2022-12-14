pub trait BasicStrings {
    fn species(&self) -> &[&str];
    fn items(&self) -> &[&str];
    fn moves(&self) -> &[&str];
    fn abilities(&self) -> &[&str];
    fn types(&self) -> &[&str];
    fn natures(&self) -> &[&str];

    fn egg_name(&self) -> &str;
}
