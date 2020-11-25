
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
      regex: Regex::new("(\\])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629365833772,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:.|(\\\\(?:[0-7]{3}|x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4}))|(\\\\c[A-Z])|(\\\\.))\\-(?:[^\\]\\\\]|(\\\\(?:[0-7]{3}|x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4}))|(\\\\c[A-Z])|(\\\\.))"),
      scope: vec![
        Scope {
            a: 59955136461799697,
            b: 12384898975268864,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955200845545516,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955200843972652,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955200847315722,
            b: 12384898975268864,
        },
    ]),(4, vec![
        Scope {
            a: 59955200845545516,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59955200843972652,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 59955200847315722,
            b: 12384898975268864,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9658 })),
]
} }