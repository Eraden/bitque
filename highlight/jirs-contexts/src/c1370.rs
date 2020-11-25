
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
        a: 844497945755648,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844497945755648,
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
      regex: Regex::new("^(.*?)\\(([0-9]+)(?:,\\s*([0-9]+))?\\): "),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130656370706,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955089199792146,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955089199792146,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Warning|Error)(:) "),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 161004842025877504,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620722290688,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }