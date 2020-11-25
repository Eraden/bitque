
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
      regex: Regex::new("^\\s*(defmodule)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636738682979,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6756 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(defprotocol)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636761620579,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6757 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(defimpl)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636761620579,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6758 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(def|defmacro)\\s+([\\p{L}_]\\w*(?:!|\\?)?)(?:(\\()|\\s*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636738682979,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630617195,
            b: 27866022694354944,
        },
    ]),(3, vec![
        Scope {
            a: 47288629327560803,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6850 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(defp|defmacrop)\\s+([\\p{L}_]\\w*(?:!|\\?)?)(?:(\\()|\\s*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636738682979,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630617196,
            b: 27866022694354944,
        },
    ]),(3, vec![
        Scope {
            a: 47288629327560803,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6850 }),
    ]),
      with_prototype: None
    }),
]
} }