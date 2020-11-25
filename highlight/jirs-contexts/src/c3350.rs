
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
      regex: Regex::new("(?x:\n  (\\[\\^)\n  ([^]]+)\n  (\\])\n)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46443487132779367,
            b: 268245652805255168,
        },
    ]),(1, vec![
        Scope {
            a: 47288629312815286,
            b: 13792273858822144,
        },
    ]),(2, vec![
        Scope {
            a: 46443487132778726,
            b: 273312412839444480,
        },
    ]),(3, vec![
        Scope {
            a: 47288629312815275,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }