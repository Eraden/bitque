
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
      regex: Regex::new("(/>)|(?:(</)\\s*(?:([_$\\p{L}][-$\\p{L}\\p{N}.]*)(?<!\\.|-)(:))?((?:[a-z][a-z0-9]*|([_$\\p{L}][-$\\p{L}\\p{N}.]*))(?<!\\.|-))?\\s*(>))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153003,
            b: 42502721483309056,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324153014,
            b: 42502721483309056,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632122742,
            b: 42502721483309056,
        },
    ]),(4, vec![
        Scope {
            a: 47288620745621655,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59392130632122519,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 61925366922608791,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288629324153003,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<)\\s*(?:([_$\\p{L}][-$\\p{L}\\p{N}.]*)(?<!\\.|-)(:))?((?:[a-z][a-z0-9]*|([_$\\p{L}][-$\\p{L}\\p{N}.]*))(?<!\\.|-))(?=((<\\s*)|(\\s+))(?!\\?)|\\/?>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 42502721483309056,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122742,
            b: 42502721483309056,
        },
    ]),(3, vec![
        Scope {
            a: 47288620745621655,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632122519,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 61925366922608791,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9769 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153003,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9770 }),
    ]),
      with_prototype: None
    }),
]
} }