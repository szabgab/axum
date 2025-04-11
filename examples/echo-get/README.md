# Echo GET - accepting Query params

Show how to accept parameters in a GET request.


## Running

```sh
cargo run -p example-echo-get
```


## GET the main page

```
$ curl http://localhost:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 131
date: Tue, 18 Mar 2025 08:04:53 GMT


    <form method="get" action="/echo">
    <input type="text" name="text">
    <input type="submit" value="Echo">
    </form>
```

## GET request with parameter

```
$ curl -i http://localhost:3000/echo?text=Hello+World!

HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 29
date: Tue, 18 Mar 2025 08:06:31 GMT

You said: <b>Hello World!</b>
```

## GET request without the parameter

```
$ curl -i http://localhost:3000/echo

HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 56
date: Tue, 18 Mar 2025 08:05:13 GMT

Failed to deserialize query string: missing field `text`
```

## GET request with parameter name but without value

```
$ curl -i http://localhost:3000/echo?text=
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 17
date: Tue, 18 Mar 2025 08:07:04 GMT

You said: <b></b>
```

