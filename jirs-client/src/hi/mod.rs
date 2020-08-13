use lazy_static::lazy_static;
use syntect::dumps::from_binary;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

lazy_static! {
    pub static ref SYNTAX_SET: SyntaxSet = from_binary(include_bytes!("./newlines.packdump"));
    pub static ref THEME_SET: ThemeSet = from_binary(include_bytes!("./all.themedump"));
}
