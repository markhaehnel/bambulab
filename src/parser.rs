use crate::Message;

pub(crate) fn parse_message(message: &paho_mqtt::Message) -> Message {
    let payload = message.payload();

    if let Ok(parsed_message) = serde_json::from_slice::<Message>(payload) {
        parsed_message
    } else {
        if let Ok(message_str) = String::from_utf8(payload.to_vec()) {
            return Message::Unknown(Some(message_str));
        }
        Message::Unknown(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message_unknown_string() {
        let message = paho_mqtt::Message::new(
            "device/123456789/report",
            r#"{ "hello": "world" }"#,
            paho_mqtt::QOS_2,
        );

        let result = parse_message(&message);

        assert_eq!(
            result,
            Message::Unknown(Some(r#"{ "hello": "world" }"#.to_string()))
        );
    }

    #[test]
    fn test_parse_message_unknown_unparseble() {
        let message =
            paho_mqtt::Message::new("device/123456789/report", vec![255, 255], paho_mqtt::QOS_2);

        let result = parse_message(&message);

        assert_eq!(result, Message::Unknown(None));
    }

    #[test]
    fn test_parse_message_print() {
        let message = paho_mqtt::Message::new(
            "device/123456789/report",
            r#"{ "print": { "bed_temper": 17.40625, "wifi_signal": "-59dBm", "command": "push_status", "msg": 1, "sequence_id": "694" } }"#,
            paho_mqtt::QOS_2,
        );

        let result = parse_message(&message);

        assert!(matches!(result, Message::Print(_)));
    }

    #[test]
    fn test_paser_message_info() {
        let message = paho_mqtt::Message::new(
            "device/123456789/report",
            r#"{
                "info":{
                    "command":"get_version",
                    "sequence_id":"0",
                    "module":[
                        {
                            "name":"ota",
                            "project_name":"C11",
                            "sw_ver":"01.04.02.00",
                            "hw_ver":"OTA",
                            "sn":"01S00C123400001"
                        }
                    ],
                    "result":"success",
                    "reason":""
                }
            }"#,
            paho_mqtt::QOS_2,
        );

        let result = parse_message(&message);

        assert!(matches!(result, Message::Info(_)));
    }
}
