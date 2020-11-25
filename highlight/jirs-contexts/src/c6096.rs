
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
      regex: Regex::new("(?i)\\b((cl|st)ac|[ls]([gli]dt|tr|msw)|clts|arpl|lar|lsl|ver[rw]|inv(d|lpg|pcid)|wbinvd)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 473722385804034048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(lock|hlt|rsm|(rd|wr)(msr|pkru|[fg]sbase)|rd(pmc|tscp?)|sys(enter|exit))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 473722385804034048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(x((save(c|opt|s)?|rstors?)(64)?|[gs]etbv))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 473722385804034048,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }