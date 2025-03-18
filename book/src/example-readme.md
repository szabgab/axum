# Readme


```toml
{{#include ../../examples/readme/Cargo.toml }}
```

```rust
{{#include ../../examples/readme/src/main.rs }}
```


## Using curl

```
$ curl http://localhost:3000/
Hello, World!
```

```
$ curl -X POST -H "Content-Type: application/json" -d '{"username":"foobar"}' http://localhost:3000/users
{"id":1337,"username":"foobar"}
```


