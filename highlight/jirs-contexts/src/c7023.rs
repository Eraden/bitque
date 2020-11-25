
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
      regex: Regex::new("\\s*(with)\\b|=|(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7067 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7069 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\?{0,1})([\\p{L}0-9\'`^._ ]+)\\s*(:)(\\s*([\\p{L}0-9\'`^._ ]+)){0,1}"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258876845490176,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632450153,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?!with|get|set\\b)\\b([\\w0-9\'`^._]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450153,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7076 })),
]
} }