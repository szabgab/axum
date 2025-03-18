# Minimal logging setup

axum uses the [tracing](https://crates.io/crates/tracing) and [tracing-subscriber](https://crates.io/crates/tracing-subscriber) for logging so we need to include both.

```toml
{{#include ../../examples/minimal-tracing/Cargo.toml}
```

```rust
{{#include ../../examples/minimal-tracing/src/main.rs}
```

When we start the application with `cargo run` we'll see line like this on the terminal:

```
2025-03-17T08:39:04.089621Z DEBUG example_minimal_tracing: listening on 127.0.0.1:3000
```

When we access the main page with a browser we'll see two more lines:

```
2025-03-17T08:39:27.044996Z TRACE axum::serve: connection 127.0.0.1:58560 accepted
2025-03-17T08:39:27.045345Z DEBUG example_minimal_tracing: in handler
```
