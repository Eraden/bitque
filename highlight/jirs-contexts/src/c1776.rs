
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
        index: 1792,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 1788 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1781 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1789 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1761 })),
]
} }