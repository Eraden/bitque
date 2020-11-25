
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
      regex: Regex::new("\\p{ascii}|(.?)"),
      scope: vec![
        Scope {
            a: 59955200847317116,
            b: 12385324177031168,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314669439100,
            b: 12385324177031168,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }