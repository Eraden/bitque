
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![],
  meta_content_scope: vec![],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 9259 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9249 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9280 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9292 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9275 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9287 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9291 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9251 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9247 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9298 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9300 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9261 })),
]
} }