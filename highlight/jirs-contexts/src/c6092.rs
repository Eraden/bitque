
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
      regex: Regex::new("(?i)\\b(pmul(ld|dq)|dpp[ds])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472600493122125824,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(movntdqa)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472600493211779072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(blendv?p[ds]|pblend(vb|w))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472600493207650304,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(p(min|max)(u[dw]|s[bd]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472600493211844608,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(round[ps][sd])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472600493211516928,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b((extract|insert)ps|p((ins|ext)(r[bdq])))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472600493211910144,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(pmov([sz]x(b[dqw]|dq|wd|wq)))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472600493185302528,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(mpsadbw|phminposuw|ptest|pcmpeqq|packusdw)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472600493116424192,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(pcmp([ei]str[im]|gtq))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 472596515961962496,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }