
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
      regex: Regex::new("(?<!\\w)([-+]?0(?:x|X)[0-9a-fA-F_]+(?:U|u|L|l|UL|Ul|uL|ul|LU|Lu|lU|lu)?)((?i:[kmgtp]b)?)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089201168515,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787021119488,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)([-+]?(?:[0-9_]+)?\\.[0-9_]+(?:(?:e|E)[0-9]+)?(?:F|f|D|d|M|m)?)((?i:[kmgtp]b)?)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461443,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787021119488,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)([-+]?0(?:b|B)[01_]+(?:U|u|L|l|UL|Ul|uL|ul|LU|Lu|lU|lu)?)((?i:[kmgtp]b)?)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089185570947,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787021119488,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)([-+]?[0-9_]+(?:e|E)(?:[0-9_])?+(?:F|f|D|d|M|m)?)((?i:[kmgtp]b)?)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461443,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787021119488,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)([-+]?[0-9_]+\\.(?:e|E)(?:[0-9_])?+(?:F|f|D|d|M|m)?)((?i:[kmgtp]b)?)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461443,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787021119488,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)([-+]?[0-9_]+[\\.]?(?:F|f|D|d|M|m))((?i:[kmgtp]b)?)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461443,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787021119488,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\w)([-+]?[0-9_]+[\\.]?(?:U|u|L|l|UL|Ul|uL|ul|LU|Lu|lU|lu)?)((?i:[kmgtp]b)?)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461443,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787021119488,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }