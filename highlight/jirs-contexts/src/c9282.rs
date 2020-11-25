
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
        a: 845051995357184,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845051995357184,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 9293 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9258 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9250 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9279 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9284 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9262 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9295 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9278 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9299 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9253 })),
]
} }