use anyhow::Result;

use bambulab::{client::Client, command::Command};

#[tokio::main]
async fn main() -> Result<()> {
    let host = "printer-ip-or-hostname";
    let access_code = "printer-access-code";
    let serial = "printer-serial-number";

    let mut client = Client::new(host, access_code, serial);

    client.connect().await?;

    client.publish(Command::PushAll).await?;

    loop {
        let message = client.poll().await?;
        println!("{message:?}");
    }
}
