# mdbook-last-changed

A preprocessor for [mdbook][] to add a page's last change date and a link to the commit on every page.

[mdbook]: https://github.com/rust-lang/mdBook

It adds a "Last change" footer, including a date and a link to the corresponding commit.
It uses the configured `git-repository-url` as the base.
