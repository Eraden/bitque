
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
      regex: Regex::new("\\/\\=|\\>\\=|\\<\\=|\\=|\\>|\\<|\\b(?i:max|min|eq|neq|eql|equalp|equal)\\b"),
      scope: vec![
        Scope {
            a: 52636628119257134,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\+|\\-|\\*|\\/|\\b(?i:mod|rem|incf|decf)\\b"),
      scope: vec![
        Scope {
            a: 52636628119191598,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:and|or|not)\\b"),
      scope: vec![
        Scope {
            a: 52636628114800686,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:logand|logior|logxor|lognor|logeqv)\\b"),
      scope: vec![
        Scope {
            a: 52636628135903278,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }