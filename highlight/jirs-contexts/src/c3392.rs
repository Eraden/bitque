
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3455 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3395 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3396 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3393 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3458 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3457 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3391 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3390 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3461 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3453 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3463 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3454 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.\\.\\."),
      scope: vec![
        Scope {
            a: 47288620736970803,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }