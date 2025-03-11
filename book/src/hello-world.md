# Hello World

The standard "Hello World" application.


* Create a new create
* Add axum and tokio with "full" feature.

```
cargo new hello-world
cd hello-world
cargo add axum
cargo add tokio -F full
```

This is how our `Cargo.toml` file looks like:

```toml
{{#include ../../examples/hello-world/Cargo.toml }}
```

We defined a function to handle a request.

```rust
{{#include ../../examples/hello-world/src/main.rs:21:25 }}
```

We need to map the GET request that arrives to `/` to be handled by this function.

```rust
{{#include ../../examples/hello-world/src/main.rs:11:11 }}
```

To run the application:

```
cargo run
```

## Checking with curl

If you are on Linux or macOS you can easily install the `curl` command that allows you to access web sites from the command line.


Accessing the main page:

```
$ curl http://localhost:3000/
<h1>Hello, World!</h1>
```

We can also observe what happens if we try to access a page that does not exist. It seems that nothing happens which is rather inconvenient.

```
$ curl http://localhost:3000/hi
```

Using the `-I` flag We can ask `curl` to also display the HTTP header the server sent us. (Using the lower-case `-i` flag `curl` would print both
the header and the content that was sent by the server.)

Accessing the main page we get `200 OK` success status.


```
$ curl -I http://localhost:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 22
date: Fri, 14 Mar 2025 08:27:44 GMT
```

Accessing a page that does not exists we get a `404 Not Found` error status.

```
$ curl -I http://localhost:3000/hi
HTTP/1.1 404 Not Found
date: Fri, 14 Mar 2025 08:27:41 GMT
```

## Improving the 404 page

Check out the example showing the [404 handler](./global-404-handler.md).

## Testing

This example, as it is now, does not expopse the router so we can't test using it.
The second best alternative is to test the handler function directly.

```rust
{{#include ../../examples/hello-world/src/main.rs:25:36 }}
```


## The full example

```rust
{{#include ../../examples/hello-world/src/main.rs }}
```

