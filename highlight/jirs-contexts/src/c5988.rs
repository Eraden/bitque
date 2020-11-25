
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 5998 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5997 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5996 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5995 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5994 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5993 })),
]
} }