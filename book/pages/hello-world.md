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

We also have two additional crates that we use to test our application.

This is how our `Cargo.toml` file looks like. As mentioned earlier, here in this book we use the `axum` located in the same repository.
You will have something like this: `axum = "0.8.1"`.


```toml
{{#include ../../examples/hello-world/Cargo.toml }}
```

In our application we need to map the path part of each URL the user might visit to a function to handle that request.
For this we need a function to handle the request and we need the map the path portion of the URL to the function that will handle it.
For example if we would like to handle the URL `https://example.org/hello/world` then we need to map `/hello/world` path to the appropriate
function in our application. In particular the address of the main page is `https://example.org/` and thus the path is `/`.

We defined a function to handle a request. To make it simple we return a static string with and HTML snippet.
The name of the function does not matter.

```rust
{{#include ../../examples/hello-world/src/main.rs:26:28 }}
```

We need to map the GET request that arrives to `/` to be handled by this function.
We put the creation of the `Router` in a separate function to make it easy to test it.

```rust
{{#include ../../examples/hello-world/src/main.rs:22:24 }}
```

Finally, we need to create our server in our `main` function.

```rust
{{#include ../../examples/hello-world/src/main.rs:9:21 }}
```

To run the application type in the following command:

```
cargo run
```

This will compile the code, run the server and print the following to the terminal:

```
listening on http://127.0.0.1:3000
```

You can then open your browser and visit that address.

You should see **Hello World!** in big letters.


## Troubleshooting

If when you run `cargo run` it gives you an error like this, then you have another process running and using the 3000 port.
Maybe another example from the axum repository. You can either find the application, shut it down and try to run this again,
or you can change the port number in this example from 3000 to 3001 or some other number and try again.

```
thread 'main' panicked at examples/hello-world/src/main.rs:17:10:
called `Result::unwrap()` on an `Err` value: Os { code: 98, kind: AddrInUse, message: "Address already in use" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

## Handling other pages

If you try to visit some other page on your server e.g. `http://127.0.0.1:3000/hi` you will get a blank page.
We'll see what happens there and how to make axum to display a [custom 404 error page](./global-404-handler.md).


## Checking with curl

The [curl](https://curl.se/) command allows you to access web sites from the command line. It is a very handy tool.

Let's see how can we use it with our web site.

Accessing the main page:

```
$ curl http://localhost:3000/
<h1>Hello, World!</h1>
```

We can also observe what happens if we try to access a page that does not exist. It seems that nothing happens which is rather inconvenient.
This is the blank page we saw earlier.

```
$ curl http://localhost:3000/hi
```

Using the `-i` flag we can ask `curl` to also display the HTTP header the server sent us.
Using the upper-case `-I` flag `curl` would print only the header that was sent by the server.
It might be more convenient.

Accessing the main page we get the following:

```
$ curl -I http://localhost:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 22
date: Fri, 14 Mar 2025 08:27:44 GMT

<h1>Hello, World!</h1>
```

The first line includes the status code. This time it is `200 OK` success status.


Accessing a page that does not exists we get a `404 Not Found` error status.

```
$ curl -I http://localhost:3000/hi
HTTP/1.1 404 Not Found
date: Fri, 14 Mar 2025 08:27:41 GMT
```

## Shutting down our local server

Once you are done with this experiment you will want to shut down this local web server. Return to the terminal where you ran it and press `Ctrl-C`.

## Editing this example

Feel free to edit this example and see what happens. However, remember that after each change you'll need to stop the server and start it again.
This is rather inconvenient. Later we'll see how to make Rust automatically recompile and restart the server every time to make some changes.

## Improving the 404 page

You might dislike the fact visiting a non-existent path returns a blank page.

Check out the example showing the [404 handler](./global-404-handler.md).

## Testing

Writing automated tests for your application can save you a lot of time down the road and you might even develop you application much
faster if instead of checking it in a browser you write test. This is especially true if you are implementing an API which is designed
to be consumed by other software anyway.


```rust
{{#include ../../examples/hello-world/src/main.rs:30:51 }}
```

## The full example

```rust
{{#include ../../examples/hello-world/src/main.rs }}
```

