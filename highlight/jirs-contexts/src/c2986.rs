
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 2980 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2984 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2963 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2983 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2985 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2956 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2982 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2989 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2988 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2987 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2962 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2965 })),
]
} }