
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
      regex: Regex::new("\""),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 1407374883553280,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7679 }),
    ]),
      with_prototype: None
    }),
]
} }