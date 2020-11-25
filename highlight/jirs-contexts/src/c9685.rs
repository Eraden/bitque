
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
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))\\s*(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450811,
            b: 42221246506598400,
        },
    ]),(2, vec![
        Scope {
            a: 47288788234665984,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788293582998,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 46444251642069142,
            b: 0,
        },
        Scope {
            a: 47288629477507254,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9535 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450198,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444251642069142,
            b: 0,
        },
        Scope {
            a: 47288629477507254,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9536 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450811,
            b: 42221246506598400,
        },
    ]),(2, vec![
        Scope {
            a: 47288788234665984,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788293582998,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[_$\\p{L}][_$\\p{L}\\p{N}]*"),
      scope: vec![
        Scope {
            a: 59392130632450198,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }