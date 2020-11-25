
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
        a: 844802887254016,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844802887254016,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 6241,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 6531 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6230 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6218 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6252 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6242 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6221 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6236 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6234 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6233 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6228 })),
]
} }