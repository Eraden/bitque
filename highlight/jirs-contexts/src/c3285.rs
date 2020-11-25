
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
        a: 114280017426907185,
        b: 0,
    },
    Scope {
        a: 46447408444473393,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 114280017426907185,
        b: 0,
    },
    Scope {
        a: 46447408444473393,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: Some(
    ClearAmount::TopN(1),
),
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 3359 })),
]
} }