
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
        a: 46453073635508224,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46453073635508224,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\w+"),
      scope: vec![
        Scope {
            a: 49258876985737216,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\) \\s* (result)  \\s*  \\(   \\s*  (\\w+) \\s* \\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52645806590984192,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258876985737216,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
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