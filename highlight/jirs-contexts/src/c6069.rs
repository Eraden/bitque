
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 6080 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6074 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6086 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6089 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6090 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6091 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6092 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6071 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6072 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6073 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6097 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6096 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6070 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6099 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6088 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6082 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6081 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6083 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6093 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6094 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6095 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6098 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6075 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6087 })),
]
} }