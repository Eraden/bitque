
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
      regex: Regex::new("\\b(?:(?:(?:(?:25[0-5])|(?:2[0-4][0-9])|(?:1[0-9][0-9])|(?:[1-9][0-9])|[0-9])\\.){3}(?:(?:25[0-5])|(?:2[0-4][0-9])|(?:1[0-9][0-9])|(?:[1-9][0-9])|[0-9]))\\b"),
      scope: vec![
        Scope {
            a: 59955089306814621,
            b: 44754624376209408,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }