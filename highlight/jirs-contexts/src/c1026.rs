
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 998 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1043 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 991 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1015 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1038 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1022 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 996 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1025 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1016 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\!\\s*important"),
      scope: vec![
        Scope {
            a: 52636787047989262,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }