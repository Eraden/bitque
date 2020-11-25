
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 5973 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5975 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5971 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5952 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5957 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5958 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5948 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5987 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5942 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5941 })),
]
} }