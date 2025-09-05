# Configuration

`mdbook-last-changed` is configured using additional options under `[output.html]`:


```toml
[output.html]
# Required: Your repository URL used in the link.
git-repository-url = "https://github.com/$user/$project"
```

* If `git-repository-url` is not configured the footer will not contain the commit and a link to it and instead only show the last changed date.
* The commit URL is constructed as `<git-repository-url>/commit/<commit-id>` where `<git-repository-url>` is the configured URL and `<commit-id>` is the commit ID fetched from the local repository.
* If the commit URL pattern is different you can set `output.html.git-commit-url` to any URL. The commit ID will be appended to that URL.
* If you want to disable adding the link set `output.html.git-commit-url = false`.

Without `git-repository-url` configured:

```HTML
<footer id="last-change">Last change: <time datetime="2023-07-09">2023-07-09</time></footer>
```

With `git-repository-url` configured:

```HTML
<footer id="last-change">Last change: <time datetime="2023-07-09">2023-07-09</time>, commit: <a href="https://github.com/$user/$project/commit/$commit">0000000</a></footer>
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

