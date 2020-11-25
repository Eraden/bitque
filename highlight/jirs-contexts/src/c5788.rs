
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 5783 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5784 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5804 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5797 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5789 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5809 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5812 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5816 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5818 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5820 })),
]
} }