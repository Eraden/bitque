
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
  prototype: Some(
    ContextId {
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\be(add|block|create|dbg(rd|wr)|extend|init|ld[bu]|pa|remove|track|wb)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 467255595280367616,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\be(enter|exit|getkey|report|resume)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 467251424867123200,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\be(aug|mod(pr|t))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 467818545233788928,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\be(accept(copy)?|modpe)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 467814374820544512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }