
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
        index: 1627,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 1601 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1626 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1605 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1610 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1611 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1612 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1613 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1614 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1615 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1622 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1599 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1551 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1570 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1498 }),
    ]),
      with_prototype: None
    }),
]
} }