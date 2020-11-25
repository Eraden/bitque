
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 5288 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5238 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5243 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5261 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5253 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5289 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5303 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5275 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5254 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5204 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5202 })),
]
} }