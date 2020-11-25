
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 281861523767296,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281861523767296,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451420828565594,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6570 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\[([+-]?\\d*)(\\:)?([+-]?\\d*)(\\,)?([+-]?\\d*)(\\:)?([+-]?\\d*)\\])?\\s*([<>v^])?\\s*(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628104642560,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955089300979802,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59955089300979802,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 59955089300979802,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 59955089300979802,
            b: 0,
        },
    ]),(9, vec![
        Scope {
            a: 52636628104642560,
            b: 0,
        },
    ]),(10, vec![
        Scope {
            a: 52636628104642560,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6571 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=^|,|\\s|\\\")([0-9.eE+-]+)(?=(?m:$)|,|\\s|\\\")"),
      scope: vec![
        Scope {
            a: 46452459313954816,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089168269312,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=^|,|\\s|\\\")([^, \\t\\\"]+)(?=(?m:$)|,|\\s|\\\")"),
      scope: vec![
        Scope {
            a: 46452463608922112,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576468426752,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\,)"),
      scope: vec![
        Scope {
            a: 46450024067497984,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628104642560,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }