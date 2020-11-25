
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 844596728823808,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844596728823808,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 2376,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2376 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2372 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2342 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2356 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2306 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2296 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2386 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2387 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2313 })),
]
} }