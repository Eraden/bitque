
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
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[(?=[\\s<>]|\\\\\\n)"),
      scope: vec![
        Scope {
            a: 46444882999640170,
            b: 0,
        },
        Scope {
            a: 61925255178485942,
            b: 29836347531329536,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 7132 }),
        ContextReference::Direct(ContextId { index: 7133 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 7134 }),
        ContextReference::Direct(ContextId { index: 7135 }),
        ContextReference::Direct(ContextId { index: 7136 }),
        ContextReference::Direct(ContextId { index: 7137 }),
    ]),
      with_prototype: None
    }),
]
} }