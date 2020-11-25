
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 844467879804928,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844467879804928,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<!--"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323039079,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 220 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</?)([-_a-zA-Z0-9:]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153191,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122727,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 221 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(&)([a-zA-Z]+|#[0-9]+|#x[0-9a-fA-F]+)(;)"),
      scope: vec![
        Scope {
            a: 59955200845349223,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325005159,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629325005159,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("&"),
      scope: vec![
        Scope {
            a: 50103314677301607,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }