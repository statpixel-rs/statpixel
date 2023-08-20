# Installation

StatPixel can only be installed on GitHub at [github.com/matteopolak/statpixel](https://github.com/matteopolak/statpixel).
In the future, it may become available on the [statpixel-rs](https://github.com/statpixel-rs) GitHub organization.

## Pre-compiled binaries

Pre-compiled binaries are not available for download as all required environment variables are injected at compile time.
If you cannot build StatPixel, all versions before 0.18 have pre-compiled binaries on the [releases](https://github.com/matteopolak/statpixel/releases) page.

## Building from source

To build StatPixel from source, you will need to have [Rust](https://www.rust-lang.org/) installed along with the [cargo](https://doc.rust-lang.org/cargo/) package manager.
Since StatPixel uses many nightly features, **you will need to install the nightly toolchain**.

You will also need to provide the following environment variables:

- `DISCORD_TOKEN`: The Discord bot token for StatPixel to use.
- `REDIS_URL`: The connection URI of the Redis server to use.
- `DATABASE_URL`: The connection URI of the PostgreSQL database to use.
- `HYPIXEL_API_KEY`: An API key for the Hypixel API.
- `TOPGG_SECRET`: The secret for the Top.gg webhook.
- `TOPGG_TOKEN`: The token for the Top.gg API.
- `REDIRECT_URI`: The redirect URI for Discord's OAuth2 flow.
- `CLIENT_ID`: The client ID for Discord's OAuth2 flow.
- `CLIENT_SECRET`: The client secret for Discord's OAuth2 flow.
- `JWT_SECRET`: The secret used to sign [JSON Web Token](https://jwt.io)s.

Once the above prerequisites are satisfied, you can build StatPixel with the following command:

```bash
cargo +nightly build --release
```

This will build StatPixel in release mode, which is recommended for production use.

If you are interested in contributing to StatPixel, you can read the [CONTRIBUTING](https://github.com/matteopolak/statpixel/blob/main/CONTRIBUTING.md) guide for more information.
