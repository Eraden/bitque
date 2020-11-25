
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
      regex: Regex::new("\\busing\\b"),
      scope: vec![
        Scope {
            a: 52636636689465344,
            b: 0,
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
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcontinue\\b"),
      scope: vec![
        Scope {
            a: 52636636701196754,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bgoto\\b"),
      scope: vec![
        Scope {
            a: 52636636701196710,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\breturn\\b"),
      scope: vec![
        Scope {
            a: 52636636701196639,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bthrow\\b"),
      scope: vec![
        Scope {
            a: 52636636701196709,
            b: 3377699720527872,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(break|case|catch|continue|default|do|else|for|goto|if|_Pragma|return|switch|throw|try|while)\\b"),
      scope: vec![
        Scope {
            a: 52636636689465344,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdelete\\b(\\s*\\[\\])?|\\bnew\\b(?!])"),
      scope: vec![
        Scope {
            a: 52636636689465344,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(and|and_eq|bitand|bitor|compl|not|not_eq|or|or_eq|xor|xor_eq|noexcept)\\b"),
      scope: vec![
        Scope {
            a: 52636628119322636,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }