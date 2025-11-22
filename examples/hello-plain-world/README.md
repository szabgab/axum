## Hello plain world!

In the very first example we try to show **Hello World!**.

The application depends on `axum` and `tokio`. The test have some other dependencies.

## Run with

```
cargo run -p example-hello-text
```

then visit `http://localhost:3000/`

You will see `<h1>Hello, World!</h1>`, yes including the HTML tags. That happens as the Content-type of the response was `text/plain`.

We can see this by using `curl` in another terminal with the `-i` flag:

```
$ curl -i http://localhost:3000
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 22
date: Tue, 15 Apr 2025 09:48:31 GMT

<h1>Hello, World!</h1>
```

In the next example we'll see how to make axum set the content type to `text/html` to convince the browser to interpret the HTML.


In the test you can see two ways to check the `Content-type`. One seems to be more simple, the other one uses `headers::ContentType`.

