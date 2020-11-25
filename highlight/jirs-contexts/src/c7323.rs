
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
      regex: Regex::new("\\b(?i:(?xi)\n(?:\n    allocatable\n  | pointer\n  | target\n  | equivalence\n  | parameter\n  | external\n  | intrinsic\n  | save\n  | optional\n  | contiguous\n  | private\n  | public\n  | protected\n)\n)\\b"),
      scope: vec![
        Scope {
            a: 48414439170703360,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (dimension)  \\b  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439170703360,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (dimension)  \\s* \\(  [^)]*  \\)  )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46453047865704448,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 48414439170703360,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (intent)  \\s* \\(  \\s*  (in|out|inout)  \\s*  \\)  )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46453043570737152,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52645493058371584,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955110784335872,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }