
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
      regex: Regex::new("^\\s*(#\\s*endif)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466374770688,
            b: 0,
        },
        Scope {
            a: 52636636717449229,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(#\\s*else)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466374770688,
            b: 0,
        },
        Scope {
            a: 52636636717449481,
            b: 3659174697238528,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 729 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 730 }),
    ]),
      with_prototype: None
    }),
]
} }