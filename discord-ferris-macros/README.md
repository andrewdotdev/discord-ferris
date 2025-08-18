# discord-ferris-macros

Procedural macros for **discord-ferris**.

> **Note:** Experimental, APIs may change.

```diff
+ #[event_handler] — register on_* Gateway events
+ Short signatures (type inference)
! More convenience attributes (planned)
- Derives / Builders
```

## Install

You **don’t need** to add this crate directly — `discord-ferris` re-exports the macros.

```toml
[dependencies]
discord-ferris = "0.x"
```

## Usage

```rust
use discord_ferris::prelude::*;

#[event_handler]
async fn on_ready(ctx, ready) {
    log!("OK", "Logged as {}", ready.user.username);
}

#[event_handler]
async fn on_message_create(ctx, mc) {
    if mc.is_bot() { return; }
    if mc.message.content == "!ping" {
        let _ = ctx.reply("PONG!").await;
    }
}
```

## Conventions

* Function names start with `on_` and map to Gateway events: `on_ready` → `READY`, `on_message_create` → `MESSAGE_CREATE`.
* Signature: `#[event_handler] async fn on_<event>(ctx[, payload])` — `payload` is inferred from the name and can be omitted.
