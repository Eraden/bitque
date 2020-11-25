
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
      regex: Regex::new("\\\\[wWsSdD]|\\."),
      scope: vec![
        Scope {
            a: 59955136461799665,
            b: 218987720859451392,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\\[pP])(\\{)([\\p{L}_]+)(?:(=)([\\p{L}_]+)?)?(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136461799665,
            b: 236157694438801408,
        },
    ]),(2, vec![
        Scope {
            a: 47288629366030518,
            b: 12384898975268864,
        },
    ]),(3, vec![
        Scope {
            a: 61925409759363116,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620737430343,
            b: 12384898975268864,
        },
    ]),(5, vec![
        Scope {
            a: 61925409759363116,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629366030507,
            b: 12384898975268864,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\([trnvf0\\\\]|c[A-Z]|x[\\da-fA-F]{2}|u[\\da-fA-F]{4}|.)"),
      scope: vec![
        Scope {
            a: 59955200847315722,
            b: 12384898975268864,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }