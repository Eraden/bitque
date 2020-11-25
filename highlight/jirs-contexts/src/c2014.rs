
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
      regex: Regex::new("(?x)\n    \\b(\n      (?:[a-z]\\w*\\.)* # Optional package specification\n      [A-Z]\\w+\\b # Class name\n      (?:<(?:[\\w, ]*)>)? # Optional Generics\n      (?:\\[\\s*\\])* # Optional brackets (array)\n    )\\b"),
      scope: vec![
        Scope {
            a: 48414576475832353,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }