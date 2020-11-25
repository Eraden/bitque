
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
      regex: Regex::new("(?=@|\\*/)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s\\*\\s+"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\G(<)caption(>)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 59392130632123121,
            b: 711005791171117056,
        },
    ]),(1, vec![
        Scope {
            a: 47288629367997663,
            b: 51239294848729088,
        },
    ]),(2, vec![
        Scope {
            a: 47288629367997663,
            b: 48143070104911872,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9728 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[^\\s@*](?:[^*]|\\*[^/])*"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 845262458650624,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }