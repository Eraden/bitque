
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
      regex: Regex::new("(?xi)\n(?:^\n  \\s* (\n    enum\n    (,\\s*bind\\(c\\))?)\n  \\s*\n  (?m:$)\n)\n"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52637379865149440,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b enumerator \\b)"),
      scope: vec![
        Scope {
            a: 52645682036932608,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (endenum) \\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52637379865149440,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }