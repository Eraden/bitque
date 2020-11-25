
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
        a: 46444217268961280,
        b: 0,
    },
    Scope {
        a: 46445132096995328,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444217268961280,
        b: 0,
    },
    Scope {
        a: 46445132096995328,
        b: 0,
    },
],
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("(?!<\\\\)\\2"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444217268961280,
            b: 0,
        },
        Scope {
            a: 46445132096995328,
            b: 0,
        },
        Scope {
            a: 47288521963733163,
            b: 13510798882111488,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }