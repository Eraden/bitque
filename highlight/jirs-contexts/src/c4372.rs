
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
      regex: Regex::new("\\bsub\\b(?!::)"),
      scope: vec![
        Scope {
            a: 48414576474128445,
            b: 0,
        },
        Scope {
            a: 52638212953276477,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4377 }),
        ContextReference::Direct(ContextId { index: 4379 }),
        ContextReference::Direct(ContextId { index: 4380 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(::)?(BEGIN|CHECK|END|INIT|UNITCHECK)(?=\\s*(?:[;#{]|(?m:$)))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314668781629,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630616306,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4377 }),
        ContextReference::Direct(ContextId { index: 4379 }),
    ]),
      with_prototype: None
    }),
]
} }