#[derive(Clone)]
pub enum NanowavePlayerEvent {
    OutputText(String),
    Position(String),
}