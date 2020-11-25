
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
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(\\bexport)\\s+)?\\b(import)\\s+([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(=)\\s*(require)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636728328343,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636717449367,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087310291393,
            b: 42502721483309056,
        },
    ]),(4, vec![
        Scope {
            a: 52636628111130775,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636636770795671,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 46445243847803031,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9752 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(\\bexport)\\s+)?\\b(import)\\s+([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(=)\\s*(?!require\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636728328343,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636717449367,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087310291393,
            b: 42502721483309056,
        },
    ]),(4, vec![
        Scope {
            a: 52636628111130775,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9753 }),
    ]),
      with_prototype: None
    }),
]
} }