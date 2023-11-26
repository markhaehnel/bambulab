#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Message {
    Info(String),
    Reconnected,
    Outgoing(String),
    Unknown(String),
}
