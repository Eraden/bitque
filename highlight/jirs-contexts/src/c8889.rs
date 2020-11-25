
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
        a: 845026225553408,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845026225553408,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 9109 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9100 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9101 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9151 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8891 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9120 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9119 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9103 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8886 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8888 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9099 })),
]
} }