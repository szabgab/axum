# Query params - GET requests

There are two main ways to send data from the browser to the server. One of them happens when the request is a `GET` request. The name of these parameters are "Query params".
They are visible in the address bar.


```toml
{{#include  ../../examples/query-params-with-empty-strings/Cargo.toml }}
```

```rust
{{#include  ../../examples/query-params-with-empty-strings/src/main.rs }}
```


Using the command line we can check how this works.

If we don't provide any parameters:

```
$ curl http://localhost:3000
Params { foo: None, bar: None }
```

We provide an integer as the value of the `foo` parameter:


```
$ curl http://localhost:3000?foo=42
Params { foo: Some(42), bar: None }
```

An integer for `foo` and a string for `bar`:

```
curl "http://localhost:3000?foo=42&bar=hello"
Params { foo: Some(42), bar: Some("hello") }
```

If we provide the name of the parameter, but not any value, then for the numerical value we still get `None`, but for the string value we get an empty string.

```
$ curl "http://localhost:3000?foo=&bar="
Params { foo: None, bar: Some("") }
```

## Full example

```rust
{{#include  ../../examples/query-params-with-empty-strings/src/main.rs }}
```


