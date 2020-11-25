
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
        a: 59392130661548053,
        b: 6192449487634432,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 59392130661548053,
        b: 6192449487634432,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 1712 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1748 })),
]
} }