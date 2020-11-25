
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![],
  meta_content_scope: vec![],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) [0-9]{4} - [0-1][0-9] - [0-3][0-9] [tT ] [0-2][0-9] : [0-5][0-9] : [0-6][0-9] (?: \\.[0-9]+ )? (?: [zZ] | [+-] [0-2][0-9] : [0-5][0-9] ) | [0-9]{4} - [0-1][0-9] - [0-3][0-9] [tT ] [0-2][0-9] : [0-5][0-9] : [0-6][0-9] (?: \\.[0-9]+ )? | [0-9]{4} - [0-1][0-9] - [0-3][0-9] | [0-2][0-9] : [0-5][0-9] : [0-6][0-9] (?: \\.[0-9]+ )?"),
      scope: vec![
        Scope {
            a: 59955136482967700,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }