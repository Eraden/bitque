//! Welcome to the syntect docs.
//!
//! Much more info about syntect is available on the [Github Page](https://github.com/trishume/syntect).
//!
//! May I suggest that you start by reading the `Readme.md` file in the main repo.
//! Once you're done with that you can look at the docs for `parsing::SyntaxSet`
//! and for the `easy` module.
//!
//! Almost everything in syntect is divided up into either the `parsing` module
//! for turning text into text annotated with scopes, and the `highlighting` module
//! for turning annotated text into styled/colored text.
//!
//! Some docs have example code but a good place to look is the `syncat` example as well as the source code
//! for the `easy` module in `easy.rs` as that shows how to plug the various parts together for common use cases.
#[macro_use]
extern crate lazy_static;

pub mod easy;
mod escape;
pub mod highlighting;
pub mod html;
pub mod parsing;
pub mod util;

use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

use crate::highlighting::{ParseThemeError, SettingsError};
pub use parsing::*;

#[derive(Debug)]
pub enum LoadingError {
    Io(IoError),
    // ParseSyntax(ParseSyntaxError, Option<String>),
    ParseTheme(ParseThemeError),
    ReadSettings(SettingsError),
    BadPath,
}

impl From<SettingsError> for LoadingError {
    fn from(error: SettingsError) -> LoadingError {
        LoadingError::ReadSettings(error)
    }
}

impl From<IoError> for LoadingError {
    fn from(error: IoError) -> LoadingError {
        LoadingError::Io(error)
    }
}

impl From<ParseThemeError> for LoadingError {
    fn from(error: ParseThemeError) -> LoadingError {
        LoadingError::ParseTheme(error)
    }
}

// impl From<ParseSyntaxError> for LoadingError {
//     fn from(error: ParseSyntaxError) -> LoadingError {
//         LoadingError::ParseSyntax(error, None)
//     }
// }

impl fmt::Display for LoadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::LoadingError::*;

        match *self {
            Io(ref error) => error.fmt(f),
            // ParseSyntax(ref error, ref filename) => {
            //     if let Some(ref file) = filename {
            //         write!(f, "{}: {}", file, error)
            //     } else {
            //         error.fmt(f)
            //     }
            // }
            ParseTheme(_) => write!(f, "Invalid syntax theme"),
            ReadSettings(_) => write!(f, "Invalid syntax theme settings"),
            BadPath => write!(f, "Invalid path"),
        }
    }
}

impl Error for LoadingError {
    fn cause(&self) -> Option<&dyn Error> {
        use crate::LoadingError::*;

        match *self {
            Io(ref error) => Some(error),
            _ => None,
        }
    }
}
