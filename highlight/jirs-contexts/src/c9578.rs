
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
      regex: Regex::new("/\\*\\*(?!/)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323038870,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9411 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(/\\*)(?:\\s*((@)internal)(?=\\s|(\\*/)))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038870,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576629252246,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47292507833237654,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9412 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^[ \\t]+)?((//)(?:\\s*((@)internal)(?=\\s|(?m:$)))?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288466114282589,
            b: 42221246506598400,
        },
    ]),(2, vec![
        Scope {
            a: 51510711028613270,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323038870,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576629252246,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47292507833237654,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9413 }),
    ]),
      with_prototype: None
    }),
]
} }