# How to run the examples?

You can run the examples either from the git repository of axum or as stand alone applications.

```
git clone https://github.com/szabgab/axum.git
cd axum
cargo run -p example-NAME
```

For example, in order to run the [hello-world](./hello-world.md) example you need to execute:

```
cargo run -p example-hello-world
```

Then you can visit the web site using this address: [http://localhost:3000/](http://localhost:3000/).

Alternatively, you can copy the content of the examples, replace the

```toml
axum = { path = "../../axum" }
```

by

```toml
axum = "0.8.1"
```

and then run

```
cargo run
```


