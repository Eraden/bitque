
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 844699808038912,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844699808038912,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 4676 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4665 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4666 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4667 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4662 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4675 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4672 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4677 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4678 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4663 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4670 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4673 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4664 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4669 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4671 })),
]
} }