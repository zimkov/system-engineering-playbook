use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::book::{Book, BookItem};
use regex::Regex;
use std::io;
use serde_json;

pub struct DrawioPreprocessor;

impl DrawioPreprocessor {
    pub fn new() -> DrawioPreprocessor {
        DrawioPreprocessor
    }

    fn convert_github_to_raw(&self, github_url: &str) -> String {
        github_url
            .replace("github.com", "raw.githubusercontent.com")
            .replace("/blob/", "/")
    }

    fn create_iframe(&self, url: &str) -> String {
        let raw_url = self.convert_github_to_raw(url);
        // ВАЖНО: Убран лишний пробел перед {}
        format!(
            r#"<iframe class="drawio-viewer" src="https://viewer.diagrams.net/?highlight=0000ff&edit=_blank&layers=1&nav=1&title=diagram&url={}"></iframe>"#,
            raw_url
        )
    }
}

impl Preprocessor for DrawioPreprocessor {
    fn name(&self) -> &str {
        "drawio"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, mdbook::errors::Error> {
        let regex = Regex::new(r"@drawio\{(https://github\.com/[^\s}]+\.drawio)\}").unwrap();
        
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = regex.replace_all(&chapter.content, |caps: &regex::Captures| {
                    self.create_iframe(&caps[1])
                }).to_string();
            }
        });

        Ok(book)
    }
}

fn main() {
    let preprocessor = DrawioPreprocessor::new();
    
    if let Err(e) = mdbook::preprocess::CmdPreprocessor::parse_input(io::stdin()) {
        eprintln!("Error parsing input: {}", e);
        std::process::exit(1);
    }

    let (ctx, book) = match mdbook::preprocess::CmdPreprocessor::parse_input(io::stdin()) {
        Ok((ctx, book)) => (ctx, book),
        Err(e) => {
            eprintln!("Error parsing input: {}", e);
            std::process::exit(1);
        }
    };

    let processed_book = match preprocessor.run(&ctx, book) {
        Ok(book) => book,
        Err(e) => {
            eprintln!("Error running preprocessor: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = serde_json::to_writer(io::stdout(), &processed_book) {
        eprintln!("Error writing output: {}", e);
        std::process::exit(1);
    }
}