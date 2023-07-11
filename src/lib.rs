use std::path::{Path, PathBuf};

use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use xshell::{cmd, Shell};

pub struct LastChanged;

impl Preprocessor for LastChanged {
    fn name(&self) -> &str {
        "last-changed"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let book_root = &ctx.root;
        let src_root = book_root.join(&ctx.config.book.src);
        let git_root = find_git(book_root)
            .expect("Couldn't find the root of this project. Not a git repository?");
        log::debug!("Book root: {}", book_root.display());
        log::debug!("Src root: {}", src_root.display());
        log::debug!("Git root: {}", git_root.display());

        let repository_string: Option<&str> = match ctx.config.get("output.html.git-repository-url")
        {
            Some(val) => {
                let url = match val {
                    toml::Value::String(s) => s,
                    _ => {
                        log::trace!("git-repository-url is not a string: {val:?}");
                        return Ok(book);
                    }
                };
                log::debug!("Repository URL: {}", url);

                if !url.contains("github.com") {
                    log::trace!("git-repository-url is not a GitHub URL: {url:?}");
                    return Ok(book);
                }

                Some(&url)
            }
            None => None,
        };

        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                log::trace!("Error when changing the book. Last result: {res:?}");
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(
                    last_changed(&git_root, &src_root, repository_string, chapter).map(|md| {
                        chapter.content = md;
                    }),
                );
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }
}

fn last_changed(
    git_root: &Path,
    src_root: &Path,
    base_url: Option<&str>,
    chapter: &mut Chapter,
) -> Result<String> {
    let content = &chapter.content;

    let footer_start = "<footer id=\"last-change\">";
    if content.contains(footer_start) {
        log::trace!("Book already contains a last-change footer. Bailing out.");
        return Ok(content.into());
    }

    let path = match chapter.path.as_ref() {
        None => {
            log::trace!("Chapter has no path. Chapter: {chapter:?}");
            return Ok(content.into());
        }
        Some(path) => path,
    };
    let path = match src_root.join(&path).canonicalize() {
        Ok(path) => path,
        Err(_) => {
            log::trace!("Cannot canonicalize path: {path:?}");
            return Ok(content.into());
        }
    };
    log::trace!("Chapter path: {}", path.display());

    let modification = get_last_modification(git_root, &path);
    let text = match modification {
        Ok((date, commit)) => match base_url {
            Some(url) => {
                let url = format!("{}/commit/{}", url, commit);
                format!(
                    "Last change: {}, commit: <a href=\"{}\">{}</a>",
                    date, url, commit
                )
            }
            None => {
                format!("Last change: {}", date)
            }
        },
        Err(e) => {
            log::trace!("No modification found for {path:?}. Error: {e:?}");
            return Ok(content.into());
        }
    };

    let content = format!("{}\n{}{}</footer>", content, footer_start, text);
    log::trace!("Adding footer: {text:?}");

    Ok(content)
}

fn find_git(path: &Path) -> Option<PathBuf> {
    let mut current_path = path;
    let mut git_dir = current_path.join(".git");
    let root = Path::new("/");

    while !git_dir.exists() {
        current_path = match current_path.parent() {
            Some(p) => p,
            None => return None,
        };

        if current_path == root {
            return None;
        }

        git_dir = current_path.join(".git");
    }

    git_dir.parent().map(|p| p.to_owned())
}

fn get_last_modification(git_dir: &Path, path: &Path) -> Result<(String, String), String> {
    let sh = Shell::new().unwrap();
    let cmd = cmd!(
        sh,
        "git --no-pager --git-dir {git_dir}/.git --work-tree {git_dir} log -1 --pretty='format:%cs %h' {path}"
    );
    log::trace!("Running command: {cmd:?}");

    let mtime = cmd.read().unwrap();

    match mtime.split_once(' ') {
        Some((date, commit)) => Ok((date.to_string(), commit.to_string())),
        None => Err("no date found".into()),
    }
}
