# Handle HEAD request

This example shows how to:
- handle HEAD requests in their own rout
- handle HEAD requests in a get route

## Running

```
cargo run -p example-handle-head-request
```

## Sending GET request to GET handler

```
$ curl -i http://localhost:3000/my-get
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
x-some-header: header from GET
content-length: 13
date: Tue, 18 Mar 2025 07:10:38 GMT

body from GET
```

## Sending HEAD request to GET handler

```
$ curl -I http://localhost:3000/my-get
HTTP/1.1 200 OK
x-some-header: header from HEAD in get-handler
content-length: 0
date: Tue, 18 Mar 2025 07:11:17 GMT
```


## Sending GET request to HEAD handler

This is not handled

```
$ curl -i http://localhost:3000/my-head

HTTP/1.1 405 Method Not Allowed
allow: HEAD
content-length: 0
date: Tue, 18 Mar 2025 07:12:12 GMT
```

## Sending HEAD request to HEAD handler

```
$ curl -I http://localhost:3000/my-head

HTTP/1.1 200 OK
x-some-header: header from HEAD in head-handler
content-length: 0
date: Tue, 18 Mar 2025 07:12:50 GMT
```



