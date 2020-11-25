
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
        a: 281629595533312,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281629595533312,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 3019,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 3010 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3004 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3002 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3001 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3000 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2999 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3008 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3006 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3005 })),
]
} }