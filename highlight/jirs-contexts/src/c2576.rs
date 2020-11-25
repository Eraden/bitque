
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
  prototype: Some(
    ContextId {
        index: 2664,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2577 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2715 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2523 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2666 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2643 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2698 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2644 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2551 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2642 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2662 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2632 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2536 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2550 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2595 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2563 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2652 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2518 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2641 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2645 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2564 })),
]
} }