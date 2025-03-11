# axum

Welcome to the axum book!

This book is mostly based on the example that can be found in the git repository of axum.

As you flip through the book you'll see that for now you can mostly see the examples without explanations and without any order.

I am working on it.

## Goals

* Provide easy way to see the examples in the axum repository with explanations.
* A way to learn using axum.
* Improve the examples (adding test, filling holes if there are any)
* The book can be found on https://axum.code-maven.com/ with the hope that one day it will be accepted as part of the official axum project
and displayed on the [Tokio](https://tokio.rs/) website.



## Conventions

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

## How to run the examples?

You can run the examples either from the git repository of axum or as stand alone applications.

```
git clone https://github.com/tokio-rs/axum.git
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


