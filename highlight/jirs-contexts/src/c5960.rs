
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 5943 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5966 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5965 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5967 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5986 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5978 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5977 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6000 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5944 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5969 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5968 })),
]
} }