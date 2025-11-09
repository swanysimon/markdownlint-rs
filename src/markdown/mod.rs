mod front_matter;
mod parser;

pub use front_matter::{detect_front_matter, FrontMatter, FrontMatterType};
pub use parser::MarkdownParser;
