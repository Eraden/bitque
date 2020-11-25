
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
        a: 46444230169723062,
        b: 1407374883553280,
    },
    Scope {
        a: 46446618152861696,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444230169723062,
        b: 1407374883553280,
    },
    Scope {
        a: 46446618152861696,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 10158 })),
]
} }