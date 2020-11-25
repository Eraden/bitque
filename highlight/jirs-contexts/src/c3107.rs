
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
        index: 3111,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[Xx])\\h*(?:(\\.)\\h*(?:[Pp][-+]?\\d*)?|(?:[Pp][-+]?\\d*))"),
      scope: vec![
        Scope {
            a: 59955089176592600,
            b: 13229323905400832,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 13229323905400832,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397935,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(0[Xx])\\h+"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 13229323905400832,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070764,
            b: 13229323905400832,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\d+(?:(\\.)\\d*(?:[Ee][-+]?\\d*)?|(?:[Ee][-+]?\\d*))|(\\.)\\d+(?:[Ee][-+]?\\d*)?"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 13229323905400832,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620735397935,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397935,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\d+"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 13229323905400832,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }