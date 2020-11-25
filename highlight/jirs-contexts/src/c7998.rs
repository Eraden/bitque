
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
        a: 844944621174784,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844944621174784,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\:"),
      scope: vec![
        Scope {
            a: 52636628106674176,
            b: 0,
        },
        Scope {
            a: 46453571712516096,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7962 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7963 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7965 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8013 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7964 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8018 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7997 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8001 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8008 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7995 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7974 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8015 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7960 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7989 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7999 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7961 })),
]
} }