
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3708 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3707 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3815 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3811 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3814 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3750 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3751 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3749 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3768 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<="),
      scope: vec![
        Scope {
            a: 52636628119257101,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }