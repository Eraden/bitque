
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
        a: 46444131394715696,
        b: 0,
    },
    Scope {
        a: 55451949099646976,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444131394715696,
        b: 0,
    },
    Scope {
        a: 55451949099646976,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 3184 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3199 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3178 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=#)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3135 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3190 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\s*;)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3137 }),
    ]),
      with_prototype: None
    }),
]
} }