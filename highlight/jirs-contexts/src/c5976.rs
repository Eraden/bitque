
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 5945 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5999 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5984 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5962 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5961 })),
]
} }