# Configuration

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

