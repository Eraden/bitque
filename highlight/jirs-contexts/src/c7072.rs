
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
      regex: Regex::new("\\b(let mutable|static let mutable|let inline|let|member val|static member inline|static member|default|member|override|let!)(\\s+rec|mutable)?(\\s+\\[\\<.*\\>\\])?\\s*(private|internal|public)?\\s+(\\[[^-=]*\\]|[_\\p{L}]([_\\p{L}0-9,\\._]+)*|``[_\\p{L}]([_\\p{L}0-9,\\._`\\s]+|(?<=,)\\s)*)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255114915945,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 49258571895930880,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7036 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(static val mutable|val mutable|val)(\\s+rec|mutable)?(\\s+\\[\\<.*\\>\\])?\\s*(private|internal|public)?\\s+(\\[[^-=]*\\]|[_\\p{L}]([_\\p{L}0-9,\\._]+)*|``[_\\p{L}]([_\\p{L}0-9,\\._`\\s]+|(?<=,)\\s)*)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255114915945,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 49258571895930880,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7037 }),
    ]),
      with_prototype: None
    }),
]
} }