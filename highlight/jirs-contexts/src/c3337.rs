
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3324 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3305 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3341 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3313 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3362 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3327 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3342 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3309 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3308 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3331 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3352 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3346 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3350 })),
]
} }