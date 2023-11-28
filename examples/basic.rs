use anyhow::Result;

use bambulab::{client::Client, command::Command, message::Message};

#[tokio::main]
async fn main() -> Result<()> {
    let host = "printer-ip-or-hostname";
    let access_code = "printer-access-code";
    let serial = "printer-serial-number";

    let (tx, mut rx) = tokio::sync::broadcast::channel::<Message>(25);

    let mut client = Client::new(host, access_code, serial, tx);
    let mut client_clone = client.clone();

    tokio::try_join!(
        tokio::spawn(async move {
            client.run().await.unwrap();
        }),
        tokio::spawn(async move {
            loop {
                let message = rx.recv().await.unwrap();
                println!("received: {message:?}");

                if message == Message::Connected {
                    client_clone.publish(Command::PushAll).await.unwrap();
                }
            }
        }),
    )?;

    Ok(())
}
