
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
            a: 46445243847803030,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?:(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(public|private|protected|readonly)\\s+)?(?:(\\.\\.\\.)\\s*)?(?<!=|:)(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(this)|([_$\\p{L}][_$\\p{L}\\p{N}]*))\\s*(\\??)(?=\\s*(:\\s*(\n  (<) |\n  ([(]\\s*(\n    ([)]) |\n    (\\.\\.\\.) |\n    ([_$\\p{L}\\p{N}]+\\s*(\n      ([:,?=])|\n      ([)]\\s*=>)\n    ))\n  ))\n)) |\n(:\\s*(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))Function(?![_$\\p{L}\\p{N}])(?:(?=\\.\\.\\.)|(?!\\.))) |\n(:\\s*((<\\s*(?m:$))|((<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*\\>)*>\\s*)?[\\(]\\s*((([\\{\\[]\\s*)?(?m:$))|((\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})\\s*((:\\s*\\{?(?m:$))|((\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+\\s*)?=\\s*)))|((\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])\\s*((:\\s*\\[?(?m:$))|((\\s*([^<>\\(\\)\\{\\}]|\\<([^<>]|\\<[^<>]+\\>)+\\>|\\([^\\(\\)]+\\)|\\{[^\\{\\}]+\\})+\\s*)?=\\s*))))))))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439033405440,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628265336982,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615190,
            b: 0,
        },
        Scope {
            a: 49259061576401046,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630615190,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636628167491734,
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
            a: 48414439033405440,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628265336982,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49258876848439296,
            b: 0,
        },
        Scope {
            a: 49259061576401046,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 49258876848439296,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636628167491734,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9677 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620732645526,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9675 })),
]
} }