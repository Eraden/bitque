
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9873 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9968 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9936 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9992 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9981 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9989 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9993 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9987 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9982 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9988 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9983 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9990 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9984 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9986 })),
]
} }