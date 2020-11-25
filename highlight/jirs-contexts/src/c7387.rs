
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
      regex: Regex::new("(?xi)\n(?:^\n  \\s* (use)\n  \\s+ (\\w+)\n  ( (,) \\s* (only) \\s* (:) )?\n)\n"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 316097296641359872,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52640644040294400,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576512534725,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 295267744564314321,
            b: 631911322715422720,
        },
    ]),(5, vec![
        Scope {
            a: 52639321190367232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }