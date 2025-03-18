{{#include  ../../examples/query-params-with-empty-strings/README.md }}

There are two main ways to send data from the browser to the server. One of them happens when the request is a `GET` request. The name of these parameters are "Query params".
They are visible in the address bar.


```toml
{{#include  ../../examples/query-params-with-empty-strings/Cargo.toml }}
```

```rust
{{#include  ../../examples/query-params-with-empty-strings/src/main.rs }}
```


## Full example

```rust
{{#include  ../../examples/query-params-with-empty-strings/src/main.rs }}
```


