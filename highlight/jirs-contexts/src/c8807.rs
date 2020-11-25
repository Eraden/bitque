
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
      regex: Regex::new("\\s*(?:([A-Z][\\w.\']*)(?:\\s+([a-z_][\\w.\']*))+ | \\(([A-Z][\\w.\']*)(?:\\s+([a-z_][\\w.\']*))+\\))\\s*(=>)"),
      scope: vec![
        Scope {
            a: 46454001210163200,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186470432903,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087300853760,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392186470432903,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 49259087300853760,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636628107591680,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }