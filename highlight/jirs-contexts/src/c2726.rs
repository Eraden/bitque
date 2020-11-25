
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
      regex: Regex::new("(\\()(?:(\\?:)|(\\?)(<)([_$\\p{L}][_$\\p{L}\\p{N}]*)(>))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582454,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 47288629318583117,
            b: 12384898975268864,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582454,
            b: 12384898975268864,
        },
    ]),(4, vec![
        Scope {
            a: 47288629318583033,
            b: 51228634739900416,
        },
    ]),(5, vec![
        Scope {
            a: 59392130633760883,
            b: 12384898975268864,
        },
    ]),(6, vec![
        Scope {
            a: 47288629318583033,
            b: 48132409996083200,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2718 }),
    ]),
      with_prototype: None
    }),
]
} }