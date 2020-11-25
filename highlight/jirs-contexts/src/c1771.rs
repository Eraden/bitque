
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
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\\\)\\\"(?=\\s*(?:#.*)?(?m:$))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445780657963029,
            b: 7036874417766400,
        },
        Scope {
            a: 55451420828565525,
            b: 7036874417766400,
        },
        Scope {
            a: 47288629323956395,
            b: 5911081885106176,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }