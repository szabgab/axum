{{#include ../../examples/echo-get/README.md }}

## Cargo.toml

```toml
{{#include ../../examples/echo-get/Cargo.toml }}
```

There are two function handling two paths:

## The main page is static HTML

```rust
{{#include ../../examples/echo-get/src/main.rs:19:28 }}
```

## The echo page

```rust
{{#include ../../examples/echo-get/src/main.rs:30:33 }}
```

## Struct describing the parameters

```rust
{{#include ../../examples/echo-get/src/main.rs:35:39 }}
```

## Mapping the routes to functions

```rust
{{#include ../../examples/echo-get/src/main.rs:13:17 }}
```


## The full example

```rust
{{#include ../../examples/echo-get/src/main.rs }}
```
