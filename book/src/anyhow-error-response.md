# anyhow-error-response

```toml
{{#include ../../examples/anyhow-error-response/Cargo.toml }}
```

```rust
{{#include ../../examples/anyhow-error-response/src/main.rs }}
```

```
$ curl -i  http://localhost:3000
HTTP/1.1 500 Internal Server Error
content-type: text/plain; charset=utf-8
content-length: 32
date: Sun, 16 Mar 2025 16:18:04 GMT

Something went wrong: it failed!
```

