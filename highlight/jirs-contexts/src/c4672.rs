
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
      regex: Regex::new("\\bif\\b(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 52636636711616746,
            b: 18014398509481984,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\belse\\b"),
      scope: vec![
        Scope {
            a: 52636636711616777,
            b: 18014398509481984,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bbreak\\b"),
      scope: vec![
        Scope {
            a: 52636636701196708,
            b: 18014398509481984,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bnext\\b"),
      scope: vec![
        Scope {
            a: 52636636701196754,
            b: 18014398509481984,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\breturn(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 52636636701196639,
            b: 18014398509481984,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\brepeat\\b"),
      scope: vec![
        Scope {
            a: 52636636708536591,
            b: 18014398509481984,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bfor\\b(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 52636636708536544,
            b: 18014398509481984,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwhile\\b(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 52636636708536556,
            b: 18014398509481984,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bin\\b"),
      scope: vec![
        Scope {
            a: 52636628119322688,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }