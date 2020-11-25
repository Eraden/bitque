
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
      regex: Regex::new("^(=====) (\\w.*)(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 114281636671850032,
            b: 23643898043695104,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629353709652,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630090836,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }