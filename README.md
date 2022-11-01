# mdbook-last-changed

A preprocessor for [mdbook][] to add a page's last change date and a link to the commit on every page.

[mdbook]: https://github.com/rust-lang/mdBook

It adds a "Last change" footer, including a date and a link to the corresponding commit.
It uses the configured `git-repository-url` as the base.

## Requirements

* The `git` command line tool.
* A configured git repository url in your `book.toml` configuration. See [Configuration](#configuration).
* Access to the git repository checkout while building your book.

## Installation

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

## Configuration

`mdbook-last-changed` is configured using additional options under `[output.html]`:


```toml
[output.html]
# Required: Your repository URL used in the link.
git-repository-url = "https://github.com/$user/$project"
```

To style the footer add a custom CSS file for your HTML output:

```toml
[output.html]
additional-css = ["last-changed.css"]
```

And in `last-changed.css` style the `<footer>` element or directly the CSS element id `last-changed`:

```css
footer {
  font-size: 0.8em;
  text-align: center;
  border-top: 1px solid black;
  padding: 5px 0;
}
```

This code block shrinks the text size, center-aligns it under the rest of the content
and adds a small horizontal bar above the text to separate it from the page content.


Finally, build your book as normal:

```
mdbook path/to/book
```

## License

MPL. See [LICENSE](LICENSE).  
Copyright (c) 2022 Jan-Erik Rediger <janerik@fnordig.de>
