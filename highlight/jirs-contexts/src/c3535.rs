
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
      regex: Regex::new("(\')([^\\\\]|\\\\(x[a-fA-F0-9][a-fA-F0-9]|[0-2]\\d\\d|[bnrt\'\"\\\\]))(\')"),
      scope: vec![
        Scope {
            a: 59955200834994176,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629382086838,
            b: 14918173765664768,
        },
    ]),(4, vec![
        Scope {
            a: 47288629382086827,
            b: 14918173765664768,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }