
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7413 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7405 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7408 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7415 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7417 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7402 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7412 })),
]
} }