use std::time::Duration;

use anyhow::Result;
use rand::Rng;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS, TlsConfiguration, Transport};

use crate::{command::Command, message::Message, parser::parse_event};

pub struct Client {
    pub host: String,
    pub access_code: String,
    pub serial: String,

    client_components: (AsyncClient, EventLoop),

    topic_device_request: String,
    topic_device_report: String,
}

impl Client {
    pub fn new<S: Into<String>>(host: S, access_code: S, serial: S) -> Self {
        let host: String = host.into();
        let access_code: String = access_code.into();
        let serial: String = serial.into();

        let client_id = format!("bambu-api-{}", rand::thread_rng().gen_range(0..100));

        let mut mqttoptions = MqttOptions::new(client_id, host.clone(), 8883);
        mqttoptions.set_transport(Transport::Tls(TlsConfiguration::Native));
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        mqttoptions.set_credentials("bblp", &access_code);

        Self {
            host,
            access_code,
            serial: serial.clone(),
            client_components: AsyncClient::new(mqttoptions, 25),
            topic_device_request: format!("device/{}/request", &serial),
            topic_device_report: format!("device/{}/report", &serial),
        }
    }

    /// Polls for a message from the MQTT event loop.
    /// You need to poll periodically to receive messages
    /// and to keep the connection alive.
    ///
    /// **NOTE** Don't block this while iterating
    ///
    /// # Errors
    ///
    /// Returns an error if there was a problem polling for a message or parsing the event.
    pub async fn poll(&mut self) -> Result<Message> {
        let (_, ref mut eventloop) = &mut self.client_components;
        let message = eventloop.poll().await?;

        parse_event(message)
    }

    /// Publishes a command to the MQTT broker.
    ///
    /// # Errors
    ///
    /// Returns an error if there was a problem publishing the command.
    pub async fn publish(&mut self, command: Command) -> Result<()> {
        let (client, _) = &mut self.client_components;
        let payload = command.get_payload();

        client
            .publish(&self.topic_device_request, QoS::AtMostOnce, false, payload)
            .await?;

        Ok(())
    }

    /// Subscribes to the MQTT broker for device reports.
    ///
    /// **NOTE** This is required to receive messages from the device.
    ///
    /// # Errors
    ///
    /// Returns an error if there was a problem subscribing to the topic.
    pub async fn subscribe(&mut self) -> Result<()> {
        let (client, _) = &mut self.client_components;

        println!("{}", self.topic_device_report);

        client
            .subscribe(self.topic_device_report.clone(), QoS::AtMostOnce)
            .await?;

        Ok(())
    }
}
