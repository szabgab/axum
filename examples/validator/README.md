# Validator

Showing how to use the [validator](https://crates.io/crates/validator) crate to validate the values passed by the users.

## Run

```
cargo run -p example-validator
```

## Test

```
cargo test -p example-validator
```

## Use

Field not supplied

```
$ curl -i http://localhost:3000/
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 48
date: Wed, 19 Mar 2025 15:00:06 GMT

Failed to deserialize form: missing field `name`
```

Input too short

```
$ curl -i http://localhost:3000/?name=
HTTP/1.1 400 Bad Request
content-type: text/plain; charset=utf-8
content-length: 48
date: Wed, 19 Mar 2025 15:03:22 GMT

Input validation error: [name: Can not be empty]
```

Acceptable input

```
$ curl -i http://localhost:3000/?name=Jo
HTTP/1.1 200 OK
content-type: text/html; charset=utf-8
content-length: 19
date: Wed, 19 Mar 2025 15:03:52 GMT

<h1>Hello, Jo!</h1>$
```


