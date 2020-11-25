
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1924 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1892 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1881 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1888 })),
]
} }