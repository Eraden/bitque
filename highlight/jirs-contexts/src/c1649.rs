
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1530 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1573 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1594 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1525 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1579 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1593 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1628 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1644 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1596 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1668 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1517 })),
]
} }