
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
      regex: Regex::new("\\b(?:make|new)\\b(?=(?:(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*\\))*(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*\\()"),
      scope: vec![
        Scope {
            a: 49258881135607808,
            b: 0,
        },
        Scope {
            a: 61925255090602015,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1938 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:bool|byte|complex64|complex128|error|float32|float64|int|int8|int16|int32|int64|rune|string|uint|uint8|uint16|uint32|uint64|uintptr)\\b(?=(?:(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*\\))*(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*\\()"),
      scope: vec![
        Scope {
            a: 49258881135607808,
            b: 0,
        },
        Scope {
            a: 61925375349686303,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:append|cap|close|complex|copy|delete|imag|len|make|new|panic|print|println|real|recover)\\b(?=(?:(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*\\))*(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*\\()"),
      scope: vec![
        Scope {
            a: 49258881135607808,
            b: 0,
        },
        Scope {
            a: 61925255090602015,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?!\\b(?:break|case|chan|const|continue|default|defer|else|fallthrough|for|func|go|goto|if|import|interface|map|package|range|return|select|struct|switch|type|var)\\b)[\\p{L}_][\\p{L}\\p{N}_]*\\b(?=(?:(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*\\))*(?:\\s|/[*](?:[^*]|[*](?!/))*[*]/)*\\()"),
      scope: vec![
        Scope {
            a: 49258881135607808,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }