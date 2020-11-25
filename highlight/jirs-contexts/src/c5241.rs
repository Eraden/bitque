
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
        index: 5277,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5240 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5260 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[%n]"),
      scope: vec![
        Scope {
            a: 59955136435322953,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%\\-?#?[bBhHsS]"),
      scope: vec![
        Scope {
            a: 59955136435322953,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%\\-?[cC]"),
      scope: vec![
        Scope {
            a: 59955136435322953,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%\\-?[tT][HIklMSLNpzZsQBbhAaCYyjmdeRTrDFc]?"),
      scope: vec![
        Scope {
            a: 59955136435322953,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[\\+\\-# 0\\(,]*[\\.0-9]*[feEgGaA]"),
      scope: vec![
        Scope {
            a: 59955136435322953,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("%[\\+\\-# 0\\(,]*[doxX]"),
      scope: vec![
        Scope {
            a: 59955136435322953,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }