
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
      regex: Regex::new("(%)(token)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787084428353,
            b: 15199648742375424,
        },
    ]),(2, vec![
        Scope {
            a: 52636787083903030,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3542 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(%)(left|right|nonassoc)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787084428353,
            b: 306807956542849024,
        },
    ]),(2, vec![
        Scope {
            a: 52636787083904066,
            b: 15199648742375424,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3543 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(%)(start)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787084428356,
            b: 15199648742375424,
        },
    ]),(2, vec![
        Scope {
            a: 52636787084099638,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3544 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(%)(type)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787084427764,
            b: 15199648742375424,
        },
    ]),(2, vec![
        Scope {
            a: 52636787045302326,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3545 }),
    ]),
      with_prototype: None
    }),
]
} }