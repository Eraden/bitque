
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
        a: 844643973464064,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844643973464064,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 3398 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3394 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3393 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3395 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3455 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3458 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3457 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3451 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3391 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3390 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3453 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3454 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3461 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3463 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3452 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3462 })),
]
} }