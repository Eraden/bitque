
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
      regex: Regex::new("(\\@)[-\\w]+\\b"),
      scope: vec![
        Scope {
            a: 46444118526132245,
            b: 0,
        },
        Scope {
            a: 59392130630419112,
            b: 5910974510923776,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322449576,
            b: 5910974510923776,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }