
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
      regex: Regex::new("(?:^\\s*|\\s+)(\\#)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711032873034,
            b: 0,
        },
        Scope {
            a: 47288629323038902,
            b: 20829148276588544,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5329 }),
    ]),
      with_prototype: None
    }),
]
} }