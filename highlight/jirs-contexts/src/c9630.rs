
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
      regex: Regex::new("\\b(?<!\\$)0(?:x|X)[0-9a-fA-F][0-9a-fA-F_]*(n)?\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 59955089201168534,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?<!\\$)0(?:b|B)[01][01_]*(n)?\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 59955089190486166,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?<!\\$)0(?:o|O)?[0-7][0-7_]*(n)?\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 59955089185570966,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?<!\\$)(?:\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*(n)?\\b)| # 1.1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[eE][+-]?[0-9][0-9_]*(n)?\\b)|             # 1.E+3\n  (?:\\B(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*(n)?\\b)|             # .1E+3\n  (?:\\b[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*(n)?\\b)|                 # 1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*(n)?\\b)|                      # 1.1\n  (?:\\b[0-9][0-9_]*(\\.)(n)?\\B)|                                  # 1.\n  (?:\\B(\\.)[0-9][0-9_]*(n)?\\b)|                                  # .1\n  (?:\\b[0-9][0-9_]*(n)?\\b(?!\\.))                                 # 1\n)(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 59955089176658070,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 46450024075888714,
            b: 42221246506598400,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ]),(3, vec![
        Scope {
            a: 46450024075888714,
            b: 42221246506598400,
        },
    ]),(4, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ]),(5, vec![
        Scope {
            a: 46450024075888714,
            b: 42221246506598400,
        },
    ]),(6, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ]),(7, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ]),(8, vec![
        Scope {
            a: 46450024075888714,
            b: 42221246506598400,
        },
    ]),(9, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ]),(10, vec![
        Scope {
            a: 46450024075888714,
            b: 42221246506598400,
        },
    ]),(11, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ]),(12, vec![
        Scope {
            a: 46450024075888714,
            b: 42221246506598400,
        },
    ]),(13, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ]),(14, vec![
        Scope {
            a: 48414576476555767,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }