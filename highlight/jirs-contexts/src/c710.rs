
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 805 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 806 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 791 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 3659174697238528,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 711 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<"),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 3659174697238528,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 712 }),
    ]),
      with_prototype: None
    }),
]
} }