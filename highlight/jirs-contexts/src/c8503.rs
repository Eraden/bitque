
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 8531 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8524 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8532 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8521 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8535 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8510 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8497 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8498 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8522 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8519 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8526 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8502 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8499 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8529 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8527 })),
]
} }