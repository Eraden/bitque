
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1571 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1584 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1640 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1641 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1663 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1655 })),
]
} }