
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
        a: 845082060128256,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845082060128256,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 10015 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10026 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10027 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10028 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10021 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10025 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10024 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10022 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10029 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10031 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10030 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10020 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10019 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10018 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10017 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10032 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10016 })),
]
} }