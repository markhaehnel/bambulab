#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Message {
    Info(String),
    Connected,
    Disconnect,
    Outgoing(String),
    Unknown(String),
}
