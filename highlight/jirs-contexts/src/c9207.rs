
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9228 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9203 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9212 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9205 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9215 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9214 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9210 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9202 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9230 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9208 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9229 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9206 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9209 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9213 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9216 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9222 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9231 })),
]
} }