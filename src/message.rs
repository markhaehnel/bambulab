#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Message {
    Info(String),
    Outgoing(String),
    Unknown(String),

    Connecting,
    Connected,
    Reconnecting,
    Disconnected,
}
