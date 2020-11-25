
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
        a: 845116419866624,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845116419866624,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 10276 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10279 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10287 })),
]
} }