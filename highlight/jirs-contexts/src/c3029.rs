
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
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 3026 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3032 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3027 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3035 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3036 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3028 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3030 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3033 })),
]
} }