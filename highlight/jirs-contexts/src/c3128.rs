
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
        a: 46444217268961280,
        b: 0,
    },
    Scope {
        a: 55451949099646976,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444217268961280,
        b: 0,
    },
    Scope {
        a: 55451949099646976,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 3172 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3199 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3170 })),
]
} }