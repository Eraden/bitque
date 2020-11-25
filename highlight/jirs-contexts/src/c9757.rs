
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
      regex: Regex::new("}|(?=\\*/)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629367997388,
            b: 48143070104911872,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\G((?=https?://)(?:[^|}\\s*]|\\*[/])+)(\\|)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087293776279,
            b: 711005791171117056,
        },
    ]),(2, vec![
        Scope {
            a: 47288620744444382,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\G((?:[^{}@\\s|*]|\\*[^/])+)(\\|)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087349549534,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620744444382,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }