
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
      regex: Regex::new("\\b(\\d+)(?i:([smhdw]))(?=[\\d\\s,\"])"),
      scope: vec![
        Scope {
            a: 46444286028480671,
            b: 6755399441055744,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 44754624376209408,
        },
    ]),(2, vec![
        Scope {
            a: 48414439044350111,
            b: 6755399441055744,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }