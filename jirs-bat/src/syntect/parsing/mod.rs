//! Everything about parsing text into text annotated with scopes.
//! The most important struct here is `SyntaxSet`, check out the docs for that.
mod parser;
pub mod syntax_definition;
mod syntax_set;
mod yaml_load;

mod regex;
mod scope;

pub use self::parser::*;
pub use self::regex::*;
pub use self::syntax_definition::*;
pub use self::syntax_set::*;
pub use self::yaml_load::*;

pub use self::scope::*;
