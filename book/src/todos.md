# TODOs

```toml
{{#include ../../examples/todos/Cargo.toml }}
```

```rust
{{#include ../../examples/todos/src/main.rs }}
```


```
$ curl -X POST -H "Content-Type: application/json" -d '{"text": "Hello World!"}'  http://localhost:3000/todos
{"id":"ccd0ebd7-f2b3-4395-bf4a-273f1d0c9851","text":"Hello World!","completed":false}

$ curl -X POST -H "Content-Type: application/json" -d '{"text": "Another item"}'  http://localhost:3000/todos
{"id":"5903e415-e162-4767-9b57-bf6583e89c3f","text":"Another item","completed":false}
```
