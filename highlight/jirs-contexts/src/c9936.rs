
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
      regex: Regex::new("\\b(?<!\\$)0(x|X)[0-9a-fA-F][0-9a-fA-F_]*\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 59955089201168535,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?<!\\$)0(b|B)[01][01_]*\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 59955089190486167,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?<!\\$)0(o|O)?[0-7][0-7_]*\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 59955089185570967,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?<!\\$)(?:\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)| # 1.1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[eE][+-]?[0-9][0-9_]*\\b)|             # 1.E+3\n  (?:\\B(\\.)[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)|             # .1E+3\n  (?:\\b[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*\\b)|                 # 1E+3\n  (?:\\b[0-9][0-9_]*(\\.)[0-9][0-9_]*\\b)|                      # 1.1\n  (?:\\b[0-9][0-9_]*(\\.)\\B)|                                  # 1.\n  (?:\\B(\\.)[0-9][0-9_]*\\b)|                                  # .1\n  (?:\\b[0-9][0-9_]*\\b(?!\\.))                                 # 1\n)(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 59955089176658071,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 46450024075888714,
            b: 42502721483309056,
        },
    ]),(2, vec![
        Scope {
            a: 46450024075888714,
            b: 42502721483309056,
        },
    ]),(3, vec![
        Scope {
            a: 46450024075888714,
            b: 42502721483309056,
        },
    ]),(4, vec![
        Scope {
            a: 46450024075888714,
            b: 42502721483309056,
        },
    ]),(5, vec![
        Scope {
            a: 46450024075888714,
            b: 42502721483309056,
        },
    ]),(6, vec![
        Scope {
            a: 46450024075888714,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }