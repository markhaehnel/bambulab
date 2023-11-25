use anyhow::Result;
use rumqttc::{Event, Packet, Publish};

use crate::message::Message;

pub(crate) fn parse_event(event: Event) -> Result<Message> {
    match event {
        Event::Incoming(incoming) => match incoming {
            Packet::Publish(message) => parse_publish(&message),
            Packet::Connect(_) => Ok(Message::Connected),
            Packet::Disconnect => Ok(Message::Disconnect),
            other => Ok(Message::Unknown(format!("{other:?}"))),
        },
        Event::Outgoing(outgoing) => Ok(Message::Outgoing(format!("{outgoing:?}"))),
    }
}

fn parse_publish(message: &Publish) -> Result<Message> {
    let payload = String::from_utf8(message.payload.to_vec())?;

    Ok(Message::Info(payload))
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use rumqttc::{Outgoing, QoS};

    use super::*;

    #[test]
    fn test_parse_event() {
        let event = Event::Outgoing(Outgoing::Subscribe(0));

        let result = parse_event(event).unwrap();

        assert_eq!(result, Message::Outgoing("Subscribe(0)".into()));
    }

    #[test]
    fn test_parse_publish() {
        let message = Publish {
            dup: false,
            qos: QoS::AtMostOnce,
            retain: false,
            topic: "device/123456789/report".to_string(),
            pkid: 0,
            payload: Bytes::from(r#"{ "hello": "world" }"#),
        };

        let result = parse_publish(&message).unwrap();

        assert_eq!(result, Message::Info(r#"{ "hello": "world" }"#.to_string()));
    }
}
