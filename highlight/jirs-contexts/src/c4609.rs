
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 4592 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4559 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(raise)\\b"),
      scope: vec![
        Scope {
            a: 52636636701197574,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4463 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(assert)\\b"),
      scope: vec![
        Scope {
            a: 52636636701196844,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(del)\\b"),
      scope: vec![
        Scope {
            a: 52636787098189886,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(print)\\b(?! *([,.()\\]}]))"),
      scope: vec![
        Scope {
            a: 52636787061424190,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(exec)\\b(?! *((?m:$)|[,.()\\]}]))"),
      scope: vec![
        Scope {
            a: 52636787058016318,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(return)\\b"),
      scope: vec![
        Scope {
            a: 52636636701196639,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(break)\\b"),
      scope: vec![
        Scope {
            a: 52636636701196708,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(continue)\\b"),
      scope: vec![
        Scope {
            a: 52636636701196754,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(pass)\\b"),
      scope: vec![
        Scope {
            a: 52636636701197596,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }