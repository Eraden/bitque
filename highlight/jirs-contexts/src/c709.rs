
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 805 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 806 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 791 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 824 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\S+"),
      scope: vec![
        Scope {
            a: 55451949097353216,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }