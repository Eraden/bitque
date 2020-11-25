
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 4869 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4864 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4926 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4917 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4863 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4918 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4958 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4957 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4922 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4952 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4868 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4870 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4910 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4948 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4947 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4911 })),
]
} }