# request fedi comments (Fediverse RFC Bot)

A fediverse bot that posts a random [Request for Comments (RFC) from the IETF](https://www.rfc-editor.org/) in regular intervals.

Written in Rust, because of course.

Publicly deployed <a rel="me" href="https://botsin.space/@rfcs">here</a>.

## Development

`cargo build`, `cargo run`, `cargo test` etc. for the basics.

A `.env` file must be filled with the following data:

```shell
# Set this to your instance's base URL
instance_url=https://mastodon.example
# Obtained when creating your OAuth2 application, see https://docs.joinmastodon.org/client/token/#app
client_id=...
client_secret=...
# Not strictly necessary, but keep it around anyways
vapid_key=...
# Obtained when authorizing the application with your account, see https://docs.joinmastodon.org/client/authorized/
account_authorization_code=...
access_token=...
# Can apparently not be retrieved via API
character_limit=500
# Configure logging (see https://docs.rs/env_logger/latest/env_logger/#enabling-logging)
RUST_LOG=debug,yaserde::de=warn,yaserde_derive=warn
```

Running in release mode is recommended, since XML parsing takes very long otherwise.
