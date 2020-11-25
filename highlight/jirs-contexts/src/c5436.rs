
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
        index: 5435,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5438 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5437 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5443 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5440 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5441 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5439 })),
]
} }