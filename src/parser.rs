use anyhow::Result;

use crate::message::Message;

pub(crate) fn parse_message(message: Option<paho_mqtt::Message>) -> Result<Message> {
    if let Some(message) = message {
        let payload_str = String::from_utf8(message.payload().to_vec())?;
        Ok(Message::Info(payload_str))
    } else {
        Ok(Message::Unknown("Unknown message".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message_some() {
        let message = paho_mqtt::Message::new(
            "device/123456789/report",
            r#"{ "hello": "world" }"#,
            paho_mqtt::QOS_2,
        );

        let result = parse_message(Some(message)).unwrap();

        assert_eq!(result, Message::Info(r#"{ "hello": "world" }"#.into()));
    }

    #[test]
    fn test_parse_message_none() {
        let result = parse_message(None).unwrap();

        assert_eq!(result, Message::Unknown("Unknown message".into()));
    }
}
