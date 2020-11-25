
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
      regex: Regex::new("(?i)\\b(com[ps][ds]|pcomu?[bdqw])\\b"),
      scope: vec![
        Scope {
            a: 50103349025702202,
            b: 448115309769457664,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(cvtp(h2ps|s2ph)|frcz[ps][ds])\\b"),
      scope: vec![
        Scope {
            a: 50103349025702202,
            b: 448115309832568832,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(fn?m((add|sub)[ps][ds])|ph(addu?(b[dqw]|w[dq]|dq)|sub(bw|dq|wd))|pma(css?(d(d|q[hl])|w[dw])|dcss?wd))\\b"),
      scope: vec![
        Scope {
            a: 50103349025702202,
            b: 448115309854457856,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(pcmov|permp[ds]|pperm|prot[bdqw]|psh[al][bdqw])\\b"),
      scope: vec![
        Scope {
            a: 50103349025702202,
            b: 448115309854785536,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }