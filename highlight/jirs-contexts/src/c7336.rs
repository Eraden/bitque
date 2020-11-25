
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7317 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7358 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7383 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7366 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7304 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7379 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7381 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7346 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7347 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7341 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7343 })),
]
} }