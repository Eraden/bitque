
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
      regex: Regex::new("(%)(prec)\\s+(([a-z][a-zA-Z0-9_]*)|(([A-Z][a-zA-Z0-9_]*)))"),
      scope: vec![
        Scope {
            a: 46448087067852800,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787084428362,
            b: 15199648742375424,
        },
    ]),(2, vec![
        Scope {
            a: 52636787084492854,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630616131,
            b: 48976877875888128,
        },
    ]),(5, vec![
        Scope {
            a: 59392130632451137,
            b: 48976877875888128,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }