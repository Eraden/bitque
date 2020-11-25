
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1522 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1666 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1649 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1676 })),
]
} }