
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
        a: 55451949198475463,
        b: 23643898043695104,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 55451949198475463,
        b: 23643898043695104,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\#\\#"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956545,
            b: 56014254810333184,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5990 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5964 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5983 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5989 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5963 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5982 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5992 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5991 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5960 })),
]
} }