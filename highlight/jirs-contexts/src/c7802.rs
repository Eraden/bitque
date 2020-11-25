
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7829 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7823 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7833 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7826 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7845 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7853 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7813 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7805 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7842 })),
]
} }