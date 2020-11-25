
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
      regex: Regex::new("(\')[^\']*(\')"),
      scope: vec![
        Scope {
            a: 55451420831973606,
            b: 41658296553177088,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 41658296553177088,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956395,
            b: 41658296553177088,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }