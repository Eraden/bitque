
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
      regex: Regex::new("(&#[xX])\\h+(;)"),
      scope: vec![
        Scope {
            a: 59955200845349080,
            b: 22517998136852480,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324873808,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288689454415952,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(&#)[0-9]+(;)"),
      scope: vec![
        Scope {
            a: 59955200845349082,
            b: 22517998136852480,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324873808,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288689454415952,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(&)[\\p{L}:_][\\p{L}\\p{N}:_.-]*(;)"),
      scope: vec![
        Scope {
            a: 59955200845349625,
            b: 22517998136852480,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324873808,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288689454415952,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }