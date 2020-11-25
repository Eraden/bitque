
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
      regex: Regex::new("(?i)\\b(mov[dq])\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 468662828430000128,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(pack(ssdw|[su]swb)|punpck[hl](bw|dq|wd))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 468661316601511936,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(p(((add|sub)(d|(u?s)?[bw]))|maddwd|mul[lh]w))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 468662751120588800,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(pcmp((eq|gt)[bdw]))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 468657180548005888,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(pandn?|px?or)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 468656888490229760,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(ps([rl]l[dwq]|raw|rad))\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 468662776890392576,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(emms)\\b"),
      scope: vec![
        Scope {
            a: 52636628119324216,
            b: 468662849904836608,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }