
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
        index: 2981,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2967 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3004 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3002 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2954 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2953 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2964 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2959 })),
]
} }