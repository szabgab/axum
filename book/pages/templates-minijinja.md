# templates-minijinja

* [minijinja](https://crates.io/crates/minijinja)

```toml
{{#include ../../examples/templates-minijinja/Cargo.toml }}
```

```rust
{{#include ../../examples/templates-minijinja/src/main.rs }}
```

```jinja
{{#include ../../examples/templates-minijinja/templates/home.jinja }}
```
```jinja
{{#include ../../examples/templates-minijinja/templates/about.jinja }}
```
```jinja
{{#include ../../examples/templates-minijinja/templates/content.jinja }}
```
```jinja
{{#include ../../examples/templates-minijinja/templates/layout.jinja }}
```

```
examples/templates-minijinja/
├── Cargo.toml
├── src
│   └── main.rs
└── templates
    ├── about.jinja
    ├── content.jinja
    ├── home.jinja
    └── layout.jinja
```

