
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1886 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1889 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1885 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1932 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1931 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1926 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1883 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1882 })),
]
} }