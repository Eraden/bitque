
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
        a: 844790008053760,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844790008053760,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 6058 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6104 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6059 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6126 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6069 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6133 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6100 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6101 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6135 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6060 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6061 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6134 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6105 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6065 })),
]
} }