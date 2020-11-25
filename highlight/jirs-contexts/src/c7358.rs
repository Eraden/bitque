
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
      regex: Regex::new("(?xi)\n(?:\\b\n  [0-9]+\n  (?! \\. (?!and|or|not|eq|lt|le|gt|ge|ne|eqv|neqv)\n    | e\n  )\n  \\b\n)\n"),
      scope: vec![
        Scope {
            a: 59955089176463557,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi)\n(?:\n  (?<!\\w)\n  (\\d+\\.\\d+ | \\d+\\. | \\.\\d+ | \\d+ ) # 1.1 or 1. or .1 or 1\n  ([ed][-+]?\\d+)?                   # exponent\n  (_\\w+)?                           # 1.0_fb format\n)\n"),
      scope: vec![
        Scope {
            a: 59955089232496837,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\.true\\.|\\.false\\.)"),
      scope: vec![
        Scope {
            a: 59955110784335872,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b (X|Z) (?xi:(\'|\") (?xi:(?xi:[0-9a-f])+)+ (\'|\"))\n    |         (?xi:(\'|\") (?xi:(?xi:[0-9a-f])+)+ (\'|\")) (X|Z) \\b\n    |      Z  (?xi:(?xi:[0-9a-f])+)       \\b\n)\n"),
      scope: vec![
        Scope {
            a: 59955089176529093,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b O (?xi:(\'|\") (?xi:(?xi:[0-8])+)+ (\'|\"))\n    |     (?xi:(\'|\") (?xi:(?xi:[0-8])+)+ (\'|\")) O \\b\n)\n"),
      scope: vec![
        Scope {
            a: 59955089185573061,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }