
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
      regex: Regex::new("\\b(?=let|end|val)|^\\s*(?m:$)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(type|and)\\s+([^=]*)(=)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787080757300,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576465936384,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288620789334068,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3484 }),
    ]),
      with_prototype: None
    }),
]
} }