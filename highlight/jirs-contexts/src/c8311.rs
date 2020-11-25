
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 8264 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8286 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8296 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8321 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8325 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8312 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8310 })),
]
} }