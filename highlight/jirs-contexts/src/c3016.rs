
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
  prototype: Some(
    ContextId {
        index: 3019,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 3007 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3013 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3012 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3000 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3015 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3018 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3014 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3017 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3006 })),
]
} }