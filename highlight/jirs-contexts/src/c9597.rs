
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9665 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9657 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9605 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9575 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9564 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9648 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9572 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9673 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9628 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9619 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9638 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9595 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9601 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9623 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9669 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9647 })),
]
} }