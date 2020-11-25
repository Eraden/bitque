
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7554 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7555 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7542 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdefined\\b"),
      scope: vec![
        Scope {
            a: 52636636696084480,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }