# Echo POST

Show how to accept parameters in a POST request.

## Running

```sh
cargo run -p example-echo-post
```

## GET the main page

```
$ curl -i http://localhost:3000/
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 132
date: Tue, 18 Mar 2025 08:21:36 GMT


    <form method="post" action="/echo">
    <input type="text" name="text">
    <input type="submit" value="Echo">
    </form>
```

## POST request setting the header and the data

```
$ curl -i -X POST \
    -H "Content-Type: application/x-www-form-urlencoded" \
    --data "text=Hello World!" \
    http://localhost:3000/echo

HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 29
date: Tue, 18 Mar 2025 08:23:51 GMT

You said: <b>Hello World!</b>
```

## POST missing parameter

```
$ curl -i -X POST \
    -H "Content-Type: application/x-www-form-urlencoded" \
    http://localhost:3000/echo

HTTP/1.1 422 Unprocessable Entity
content-type: text/plain; charset=utf-8
content-length: 53
date: Tue, 18 Mar 2025 08:25:39 GMT

Failed to deserialize form body: missing field `text`
```
