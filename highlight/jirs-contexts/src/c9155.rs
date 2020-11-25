
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
      regex: Regex::new("([a-zA-Z_-]+)\\s+(\\+)\\s+([a-zA-Z_-]+(?=\\s|(?m:$)|;))|(\\+)\\s+([a-zA-Z_-]+(?=\\s|(?m:$)|;))|([a-zA-Z_-]+)\\s+(\\+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451949097418752,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628107984896,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55451949097418752,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636628107984896,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 55451949097418752,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 55451949097418752,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 52636628107984896,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }