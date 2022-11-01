# Installation

If you want to use only this preprocessor, install the tool:

```
cargo install mdbook-last-changed
```

Add it as a preprocessor to your `book.toml`:

```toml
[preprocessor.last-changed]
command = "mdbook-last-changed"
renderer = ["html"]
```
