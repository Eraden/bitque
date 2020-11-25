
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 636 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 559 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 558 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 673 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 662 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 672 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 606 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 607 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 605 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 623 })),
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