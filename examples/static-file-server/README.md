# Static File Server

## Run

```
cargo run -p example-static-file-server
```

## Explanation

This example starts to listen on 7 ports at the same time showing 7 different ways to serve static files.
When serving the files the content type is set based on the file extension.

```
.html => text/html
.js   => text/javascript
```

## using_serve_dir on port 3001

```
$ curl http://localhost:3001/assets/script.js

console.log("Hello, World!");
```

```
curl http://localhost:3001/assets/script.js   returns assets/script.js
curl http://localhost:3001/assets/index.html  returns assets/index.html
http://localhost:3001/assets/                 returns assets/index.html
```

## using_serve_dir_with_assets_fallback on port 3002

```
curl http://localhost:3002/assets/index.html   returns assets/index.html
curl http://localhost:3002/assets/script.js    returns assets/script.js
curl http://localhost:3002/assets/other        return  assets/index.html (and status code 404 Not Found)
```

