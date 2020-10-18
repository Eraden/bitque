use lazy_static::lazy_static;
use syntect::{dumps::*, highlighting::ThemeSet};

pub mod syntax_set;
//
// pub use syntax::get_syntax;
// pub use syntax::SYNTAX_LIST;
// pub use syntax::SYNTAX_NAMES;

lazy_static! {
    pub static ref THEME_SET: ThemeSet = from_binary(include_bytes!("./all.themedump"));
    // pub static ref SYNTAX_SET: SyntaxSet = syntax_set::load();
}
