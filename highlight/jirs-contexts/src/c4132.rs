
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
      regex: Regex::new("(\\$+)(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 46444883060785210,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514490,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087295807488,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4115 }),
    ]),
      with_prototype: None
    }),
]
} }