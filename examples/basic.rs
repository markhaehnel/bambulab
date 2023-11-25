use anyhow::Result;

use bambulab::{client::Client, command::Command};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::new("<PRINTER-IP>", "<PRINTER-ACCESS-CODE>", "<PRINTER-SERIAL>");

    client.subscribe().await?;

    client.publish(Command::PushAll).await?;

    loop {
        let message = client.poll().await?;
        println!("{message:?}");
    }
}
