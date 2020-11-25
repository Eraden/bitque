
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
      regex: Regex::new("(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521962160299,
            b: 36873221949095936,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?:\\\'|\\\")?)(\\w+)((?:\\\'|\\\")?)(?:\\s+)?(=)(?:\\s+)?"),
      scope: vec![
        Scope {
            a: 46453838004945027,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 36873221949095936,
        },
    ]),(2, vec![
        Scope {
            a: 49259087310291075,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956395,
            b: 36873221949095936,
        },
    ]),(4, vec![
        Scope {
            a: 52636628111130755,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8601 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8599 })),
]
} }