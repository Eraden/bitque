
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
        a: 46443865171558576,
        b: 20829148276588544,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46443865171558576,
        b: 20829148276588544,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 5435,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5413 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5407 })),
]
} }