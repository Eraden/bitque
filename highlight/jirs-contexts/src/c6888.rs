
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
      regex: Regex::new("(\\\\[dsw])|(\\\\x\\h\\h?|\\\\x{\\h+}|\\\\[0-7]{1,3}|\\o{[0-7]+}|\\\\c\\p{ascii}|\\\\.|[^\\\\])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314671534124,
            b: 27866022694354944,
        },
    ]),(2, vec![
        Scope {
            a: 59955136424902700,
            b: 27866022694354944,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }