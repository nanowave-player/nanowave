#[derive(Clone)]
pub enum NanowavePlayerEvent {
    UpdateStatus(String),
    OutputText(String),
    Position(String),
}