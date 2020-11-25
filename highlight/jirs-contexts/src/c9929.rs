
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9936 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9865 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9935 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9995 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9937 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9858 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9976 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9970 })),
]
} }