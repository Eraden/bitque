
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
      regex: Regex::new("(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956395,
            b: 14636698788954112,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\(?m:$)[ \\t]*"),
      scope: vec![
        Scope {
            a: 47288620734022685,
            b: 14636698788954112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\(x[a-fA-F0-9][a-fA-F0-9]|[0-2]\\d\\d|[bnrt\'\"\\\\])"),
      scope: vec![
        Scope {
            a: 59955200844431601,
            b: 14636698788954112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[\\|\\(\\)1-9$^.*+?\\[\\]]"),
      scope: vec![
        Scope {
            a: 59955200834404593,
            b: 14636698788954112,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\(?!(x[a-fA-F0-9][a-fA-F0-9]|[0-2]\\d\\d|[bnrt\'\"\\\\]|[\\|\\(\\)1-9$^.*+?\\[\\]]|(?m:$)[ \\t]*))(?:.)"),
      scope: vec![
        Scope {
            a: 50103314669371589,
            b: 67835469387268096,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }