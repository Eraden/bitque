
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
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(\\bexport)\\s+)?(?:(\\bdeclare)\\s+)?\\b(import)(?:\\s+(type))?\\s+([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(=)\\s*(require)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636728328342,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636636717449366,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636636702113942,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 49259087310291393,
            b: 42221246506598400,
        },
    ]),(6, vec![
        Scope {
            a: 52636628111130774,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 52636636770795670,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 46445243847803030,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9454 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(\\bexport)\\s+)?(?:(\\bdeclare)\\s+)?\\b(import)(?:\\s+(type))?\\s+([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(=)\\s*(?!require\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636728328342,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636636717449366,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636636702113942,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 49259087310291393,
            b: 42221246506598400,
        },
    ]),(6, vec![
        Scope {
            a: 52636628111130774,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9455 }),
    ]),
      with_prototype: None
    }),
]
} }