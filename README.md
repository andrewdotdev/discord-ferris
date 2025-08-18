# 🦀 discord-ferris 🦀

discord-ferris is a Discord API Rust library under development.

> \[!WARNING]
> This framework is **under development**. Features may be incomplete or unstable. Not recommended for production use.

```diff
+ client
+ gateway
+ http
+ macros
- cache
- builders
- ...
```

## Install

```sh
cargo add discord-ferris
```

Or add it to the `Cargo.toml`:

```toml
[dependencies]
discord-ferris = "x.y.z"
```

## Try the example

The `ferris-example` is a temporary development entry point and **will be removed in future versions**.

> \[!IMPORTANT]
> You need **Rust 1.85.1 or higher**. Also, copy `.env.example` to `.env` and replace `your_bot_token` with your actual Discord bot token.

```bash
cargo run -p discord-ferris --example ferris-example --features examples
```