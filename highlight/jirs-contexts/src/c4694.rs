
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
        a: 281629597958209,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281629597958209,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 4695,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 4697 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4696 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4693 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2979 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2952 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2958 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4692 })),
]
} }