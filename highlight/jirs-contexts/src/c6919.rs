
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 6928 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6929 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6904 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6903 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6911 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6917 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6909 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6922 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6905 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6920 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6925 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6927 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6916 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6923 })),
]
} }