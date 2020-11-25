
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
      regex: Regex::new(",?\\s*([A-Z][\\w\']*)\\s*(?:\\(([A-Z][\\w\\s,\']*|..)\\))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450183,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615404,
            b: 37999121855938560,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(",?\\s?class\\s+([A-Z][\\w\']*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130779971719,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(",?\\s?([\\w\']*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615040,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }