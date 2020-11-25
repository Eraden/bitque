
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 10197 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10182 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10170 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10187 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10174 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10178 })),
]
} }