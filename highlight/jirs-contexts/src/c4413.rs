
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
  uses_backrefs: true,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 4568 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4566 })),
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("\\2"),
      scope: vec![
        Scope {
            a: 47288629323038891,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }