
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 844613911511040,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844613911511040,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2721 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2722 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2729 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2728 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2725 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2726 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2723 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2724 })),
]
} }