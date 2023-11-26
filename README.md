# Bambulab API

[![Docs](https://img.shields.io/docsrs/bambulab)](https://docs.rs/bambulab)
[![GitHub workflow status](https://github.com/markhaehnel/bambulab/actions/workflows/cicd.yaml/badge.svg)](https://github.com/markhaehnel/bambulab/actions/workflows/cicd.yaml)
[![Crates.io Version](https://img.shields.io/crates/v/bambulab)](https://crates.io/crates/bambulab)
[![Crates.io Downloads](https://img.shields.io/crates/d/bambulab)](https://crates.io/crates/bambulab)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

🚧 **WORK IN PROGRESS** 🚧

bambulab is a asnyc Rust crate that provides an client for interacting with Bambu Lab devices.

## Features

- Subscribe to printer messages like print progress, temperatures, errors, etc.
- Publish commands to the printer
- Support for P1P, P1S and X1C printer

## Usage

First, add `bambulab` to your dependencies:
```bash
cargo add bambulab
```

Then, use the `Client` struct to create a new client and connect to a printer:

```rust
use bambulab::{client::Client, command::Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
```

Please note that you need to call subscribe() to allow the API to listen to messages.

More examples available in the [examples](./../examples) directory.

## FAQ

### How do I find the access code?

The access code is a 6 digit code that is printed on the back of your printer. It is also available in the Bambu Lab app.

### How do I find the serial number?

The serial number is a 6 digit code that is printed on the back of your printer. It is also available in the Bambu Lab app.

## Contributing

See the [contributing guidelines](./../CONTRIBUTING.md) for more information.

## License

This code is licensed under either of

- [MIT License](./../LICENSE-MIT)
- [Apache-2.0 License](./../LICENSE-APACHE)

at your option.

## Disclaimer

This project is not officially associated with Bambu Lab. It is a third-party implementation.