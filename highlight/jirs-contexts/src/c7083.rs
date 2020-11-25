
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
      regex: Regex::new("(%0?-?(\\d+)?((a|t)|(\\.\\d+)?(f|F|e|E|g|G|M)|(b|c|s|d|i|x|X|o|u)|(s|b|O)|(\\+?A)))"),
      scope: vec![
        Scope {
            a: 59392130632451294,
            b: 623467524385669120,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52641172319305833,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }