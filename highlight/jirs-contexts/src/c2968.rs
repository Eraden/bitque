
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
        a: 281629597958144,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281629597958144,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 2981,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2986 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2975 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2979 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2952 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2958 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2961 })),
]
} }