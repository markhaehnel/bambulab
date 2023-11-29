use std::fmt;

pub enum SpeedProfile {
    Silent,
    Standard,
    Sport,
    Ludicrous,
}

impl fmt::Display for SpeedProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Silent => write!(f, "silent"),
            Self::Standard => write!(f, "standard"),
            Self::Sport => write!(f, "sport"),
            Self::Ludicrous => write!(f, "ludicrous"),
        }
    }
}
