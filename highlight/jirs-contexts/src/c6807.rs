
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
        a: 55451420830270024,
        b: 27866022694354944,
    },
    Scope {
        a: 59955136445350472,
        b: 27866022694354944,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 55451420830270024,
        b: 27866022694354944,
    },
    Scope {
        a: 59955136445350472,
        b: 27866022694354944,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s+"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6818 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6849 })),
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("(\\2)(a)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451420830270024,
            b: 27866022694354944,
        },
        Scope {
            a: 47288629323956395,
            b: 27866022694354944,
        },
    ]),(2, vec![
        Scope {
            a: 55451420956557411,
            b: 0,
        },
        Scope {
            a: 48414576475439203,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }