
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
      regex: Regex::new("^\\s*(tell)\\s+(?=app(lication)?\\s+\"(?i:textmate)\")(?!.*\\bto(?!\\s+tell)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636707160072,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 124 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(tell)\\s+(?=app(lication)?\\s+\"(?i:finder)\")(?!.*\\bto(?!\\s+tell)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636707160072,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 125 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(tell)\\s+(?=app(lication)?\\s+\"(?i:system events)\")(?!.*\\bto(?!\\s+tell)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636707160072,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 126 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(tell)\\s+(?=app(lication)?\\s+\"(?i:itunes)\")(?!.*\\bto(?!\\s+tell)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636707160072,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 127 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(tell)\\s+(?=app(lication)?\\s+process\\b)(?!.*\\bto(?!\\s+tell)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636707160072,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 128 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(tell)\\s+(?=app(lication)?\\b)(?!.*\\bto(?!\\s+tell)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636707160072,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 129 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(tell)\\s+(?!.*\\bto(?!\\s+tell)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636707160072,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 130 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(tell)\\s+(?=.*\\bto\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636707160072,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 131 }),
    ]),
      with_prototype: None
    }),
]
} }