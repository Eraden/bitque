
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
      regex: Regex::new("^(\\s*)((#).*(?m:$)\\n?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288466114282589,
            b: 27303072740933632,
        },
    ]),(2, vec![
        Scope {
            a: 51510337349877760,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323038817,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }