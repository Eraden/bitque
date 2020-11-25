
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
        index: 5277,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5266 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5236 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5278 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5212 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5267 })),
]
} }