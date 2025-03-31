# Notes

This book uses the examples from the source code of the axum project. For this reason the `Cargo.toml` files will declare their dependency on
`axum` in a relative manner using the following entry:

```toml
axum = { path = "../../axum" }
```

This works well as the examples are included the in the [git repository of axum](https://github.com/tokio-rs/axum/). In real-world applications
this line need to be replaced by a line declaring the version of axum you'd like to use. For example:

```toml
axum = "0.8.1"
```


