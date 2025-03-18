# Redirect

We can redirect a request to another page on our side or to a page on another site.
For this we use the [Redirect struct](https://docs.rs/axum/latest/axum/response/struct.Redirect.html) that has methods
for `permanent` redirection (`308 Permanent Redirect`) and `temporary` redirection (`307 Temporary Redirect`).

```toml
{{#include ../../examples/redirect/Cargo.toml }}
```

```rust
{{#include ../../examples/redirect/src/main.rs }}
```
