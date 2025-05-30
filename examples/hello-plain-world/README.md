## Hello plain world!



Run with

```
cargo run -p example-hello-text
```

then visit http://localhost:3000/

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

In the next example we'll see how to tell 
