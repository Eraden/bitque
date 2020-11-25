
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 4372 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4338 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4311 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4310 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4282 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4333 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4335 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4383 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4326 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4381 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4275 })),
]
} }