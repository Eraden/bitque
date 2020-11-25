
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
      regex: Regex::new("^(?x:\n  (?:[0-9A-Za-z+/]{4})+\n  (?:[0-9A-Za-z+/]{2}==|\n     [0-9A-Za-z+/]{3}=)?\n)(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46444329079210088,
            b: 0,
        },
        Scope {
            a: 55451949103316992,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }