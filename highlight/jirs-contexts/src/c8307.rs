
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
        a: 844953211109376,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844953211109376,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 8277 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8278 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8323 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8313 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8295 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8294 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8281 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8301 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8266 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8280 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(,)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620722028544,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }