
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 5985 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5990 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5964 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5983 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5981 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5989 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5963 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5982 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5980 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5992 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5991 })),
]
} }