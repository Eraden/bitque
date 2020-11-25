
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 10276 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10283 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10285 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10295 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10272 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10273 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10277 })),
]
} }