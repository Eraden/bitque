
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 51510878542233641,
        b: 0,
    },
    Scope {
        a: 281496454234112,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 51510878542233641,
        b: 0,
    },
    Scope {
        a: 281496454234112,
        b: 0,
    },
],
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*/"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 51510878542233641,
            b: 0,
        },
        Scope {
            a: 47288629323038891,
            b: 11540474045136896,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }