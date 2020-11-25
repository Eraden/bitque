
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
        a: 844472174772224,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844472174772224,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: Some(
    ClearAmount::TopN(2),
),
  prototype: Some(
    ContextId {
        index: 399,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 382 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 403 })),
]
} }