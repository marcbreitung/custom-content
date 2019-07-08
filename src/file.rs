pub trait SaveFile {
    fn content(&self) -> String;
    fn file(&self) -> String;
}