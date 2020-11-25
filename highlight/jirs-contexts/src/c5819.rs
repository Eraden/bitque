
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
        a: 281818574094336,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281818574094336,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5820 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5785 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5784 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5783 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5833 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5817 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5822 })),
]
} }