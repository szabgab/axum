# Query params - GET requests

Shows hoe to get the Query parameters from a request.

From

```
https://example.org/some/path?name=Foo&height=1.87
```

extract

```
name:  Foo
height:  1.87
```


## Run

cargo run -p example-query-params-with-empty-strings

## Test

cargo test -p example-query-params-with-empty-strings

## Use

Using the command line we can check how this works.

If we don't provide any parameters:

```
$ curl http://localhost:3000
Params { foo: None, bar: None }
```

We provide an integer as the value of the `foo` parameter:


```
$ curl http://localhost:3000?foo=42
Params { foo: Some(42), bar: None }
```

An integer for `foo` and a string for `bar`:

```
curl "http://localhost:3000?foo=42&bar=hello"
Params { foo: Some(42), bar: Some("hello") }
```

If we provide the name of the parameter, but not any value, then for the numerical value we still get `None`, but for the string value we get an empty string.

```
$ curl "http://localhost:3000?foo=&bar="
Params { foo: None, bar: Some("") }
```


