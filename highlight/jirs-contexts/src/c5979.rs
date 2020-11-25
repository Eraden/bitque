
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
        a: 281835753963520,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281835753963520,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5976 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5959 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5988 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5974 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5970 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5960 })),
]
} }