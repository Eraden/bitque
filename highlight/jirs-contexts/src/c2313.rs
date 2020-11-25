
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
        index: 2376,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2318 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2303 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2352 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2389 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2297 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2349 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2355 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2393 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2290 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2348 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2315 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2374 })),
]
} }