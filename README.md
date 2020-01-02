# Slack Greeter

A simple Slack bot written in Rust that greets new members of a workspace via
direct message.

## Modus Operandi

The bot uses the `slack-rs` library to connect to Slack's RTM (Real Time
Messaging) API to receive events. Since every member of a workspace is
implicitly added to the `#general` channel and nobody can leave it, we can just
listen for new members in that channel as an indication of a new member. After
that, it formats a message and direct messages the user using the RTM API. It
would be better to use the `postMessage` HTTP API method, but RTM works for this
purpose and is easier to use with `slack-rs`'s wrapper.

## Installation

### Using Cargo

Run `cargo build --release` in a local clone of the repository and move
`target/release/slack-greeter` to the desired location.

### Using Docker

Run `docker run --rm -v "$PWD/config.toml:/data/config.toml" kdrag0n/slack-greeter`
to start the bot after configuration.

## Configuration

Copy `config.example.toml` to `config.toml` and fill everything in. Each option
is documented by the comments above it, so it should be fairly self-explanatory.
A bot token can be obtained from the [Slack website](https://my.slack.com/services/new/bot).
 