
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
        a: 844875901698048,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844875901698048,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 7070 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7067 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7071 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7084 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7066 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7073 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7072 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7062 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7064 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7079 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7063 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7074 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7080 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7082 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7085 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7076 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7065 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7087 })),
]
} }