
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
      regex: Regex::new("\\)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445243847803031,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9879 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?:(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(public|private|protected|readonly)\\s+)?(?:(\\.\\.\\.)\\s*)?(?<!=|:)(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(this)|([_$\\p{L}][_$\\p{L}\\p{N}]*))\\s*(\\??)(?=\\s*(:\\s*(\n  (<) |\n  ([(]\\s*(\n    ([)]) |\n    (\\.\\.\\.) |\n    ([_$\\p{L}\\p{N}]+\\s*(\n      ([:,?=])|\n      ([)]\\s*=>)\n    ))\n  ))\n)) |\n(:\\s*((<\\s*(?m:$))|([\\(]\\s*([\\{\\[]\\s*)?(?m:$)))))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439033470976,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628265336983,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615191,
            b: 0,
        },
        Scope {
            a: 49259061576401047,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630615191,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636628167491735,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?:(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(public|private|protected|readonly)\\s+)?(?:(\\.\\.\\.)\\s*)?(?<!=|:)(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(this)|([_$\\p{L}][_$\\p{L}\\p{N}]*))\\s*(\\??)(?=:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439033470976,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628265336983,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49258876848504832,
            b: 0,
        },
        Scope {
            a: 49259061576401047,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 49258876848504832,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636628167491735,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9979 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620732645527,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9977 })),
]
} }