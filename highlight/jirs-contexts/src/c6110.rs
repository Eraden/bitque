
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
        a: 46444328951677013,
        b: 24488322973827072,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444328951677013,
        b: 24488322973827072,
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 6109 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6118 })),
]
} }