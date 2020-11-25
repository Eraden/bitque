
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
        a: 845107840417792,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845107840417792,
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
      regex: Regex::new("^-----BEGIN [\\w ]+ PRIVATE KEY-----"),
      scope: vec![
        Scope {
            a: 47288521951477942,
            b: 44755208491761664,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^-----END [\\w ]+ PRIVATE KEY-----"),
      scope: vec![
        Scope {
            a: 47288521951477942,
            b: 44755208491761664,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^[a-zA-Z0-9+/]{1,70}(=){0,3}(?m:$)"),
      scope: vec![
        Scope {
            a: 55451949106987008,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956395,
            b: 45035996273704960,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }