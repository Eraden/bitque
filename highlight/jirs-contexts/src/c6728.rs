
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
      regex: Regex::new("\\\\[^\'\"]"),
      scope: vec![
        Scope {
            a: 59955200913376371,
            b: 27021597764222976,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6704 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*|\\+|\\?|\\.|\\|"),
      scope: vec![
        Scope {
            a: 52636787087179872,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\^|\\$"),
      scope: vec![
        Scope {
            a: 52636787087179872,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\."),
      scope: vec![
        Scope {
            a: 59955136481656928,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[(\\^)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787067585651,
            b: 27021597764222976,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6705 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{(?:\\d+)?,(?:\\d+)?\\}"),
      scope: vec![
        Scope {
            a: 52636787087179872,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }