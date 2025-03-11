# Form - accepting POST request

In this example we can see how an application can accept http POST requestts


```toml
{{#include ../../examples/form/Cargo.toml }}
```

```rust
{{#include ../../examples/form/src/main.rs }}
```

To see the responses using `curl`:

Asking for the main page with a GET request
```
curl http://localhost:3000/
```

This returns the HTML page.


```
curl -X POST -H "Content-Type: application/x-www-form-urlencoded" --data "name=Foo&email=foo@bar.com" http://localhost:3000/
email='foo@bar.com'
name='Foo'
```


Missing field:

```
curl -X POST -H "Content-Type: application/x-www-form-urlencoded" --data "name=Foo" http://localhost:3000/
Failed to deserialize form body: missing field `email`
```

Extra fields: Does not matter

```
$ curl -X POST -H "Content-Type: application/x-www-form-urlencoded" --data "name=Foo&email=foo@bar.com&age=42" http://localhost:3000/
email='foo@bar.com'
name='Foo'
```
