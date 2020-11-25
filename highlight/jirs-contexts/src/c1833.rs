
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
      regex: Regex::new("(?:d|drop)\\b"),
      scope: vec![
        Scope {
            a: 52636628100252340,
            b: 5911103359942656,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:e|edit)\\b"),
      scope: vec![
        Scope {
            a: 52636628100252341,
            b: 5911103359942656,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:x|exec)\\b"),
      scope: vec![
        Scope {
            a: 52636628100252342,
            b: 5911103359942656,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:f|fixup)\\b"),
      scope: vec![
        Scope {
            a: 52636628100252343,
            b: 5911103359942656,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:p|pick)\\b"),
      scope: vec![
        Scope {
            a: 52636628100252344,
            b: 5911103359942656,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:r|reword)\\b"),
      scope: vec![
        Scope {
            a: 52636628100252345,
            b: 5911103359942656,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:s|squash)\\b"),
      scope: vec![
        Scope {
            a: 52636628100252346,
            b: 5911103359942656,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1837 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\S+"),
      scope: vec![
        Scope {
            a: 50103314699452437,
            b: 8444249301319680,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }