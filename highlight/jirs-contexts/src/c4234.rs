
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
      regex: Regex::new("(\\b[_\\p{L}]\\w*\\b)(::)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130643525693,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788251050045,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4382 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4298 })),
]
} }