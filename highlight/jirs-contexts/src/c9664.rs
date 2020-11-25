
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9581 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9579 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9560 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9580 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9622 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9593 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9654 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9665 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
]
} }