
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 46446119994065778,
        b: 10414574138294272,
    },
    Scope {
        a: 46446119949107205,
        b: 10414574138294272,
    },
    Scope {
        a: 281496464326656,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46446119994065778,
        b: 10414574138294272,
    },
    Scope {
        a: 46446119949107205,
        b: 10414574138294272,
    },
    Scope {
        a: 281496464326656,
        b: 0,
    },
],
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("(\\})|(\\4)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }