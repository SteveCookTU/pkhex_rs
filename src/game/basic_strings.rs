pub trait BasicStrings {
    fn species(&self) -> &[String];
    fn items(&self) -> &[String];
    fn moves(&self) -> &[String];
    fn abilities(&self) -> &[String];
    fn types(&self) -> &[String];
    fn natures(&self) -> &[String];

    fn egg_name(&self) -> &str;
}
