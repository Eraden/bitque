
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
        a: 844485059674112,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844485059674112,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 998 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1035 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 986 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1024 })),
]
} }