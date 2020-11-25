
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
      regex: Regex::new("(\\B\\-|\\b)(0b[01][01_]*)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 41095346599755776,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\B\\-|\\b)(0o[0-7][0-7_]*)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 41095346599755776,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\B\\-|\\b)([0-9][0-9_]*)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 41095346599755776,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\B\\-|\\b)(0x\\h[\\h_]*)\\b"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 41095346599755776,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }