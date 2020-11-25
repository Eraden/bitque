
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 10249 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10250 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10306 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10252 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10217 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10221 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10219 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("="),
      scope: vec![
        Scope {
            a: 52636628111130781,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620756172957,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }