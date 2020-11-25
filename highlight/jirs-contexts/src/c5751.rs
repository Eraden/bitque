
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
      regex: Regex::new("\\b(proc)\\s+((?:\\$\\{[^ \\}]+\\}|[^\\s[;{}\\[\\]\"\\\\]][^\\s$[;{}\\[\\]\"\\\\]]*)+)"),
      scope: vec![
        Scope {
            a: 46444131371581440,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787017646080,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630615118,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5754 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bproc\\b(?=\\s)"),
      scope: vec![
        Scope {
            a: 52636787017646080,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }