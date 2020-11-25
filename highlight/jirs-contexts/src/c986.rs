
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 975 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 976 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 977 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 978 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 979 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 980 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 981 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 982 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 983 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 984 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 987 })),
]
} }