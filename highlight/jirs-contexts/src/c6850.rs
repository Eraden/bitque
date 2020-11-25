
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
      regex: Regex::new("(\\bdo:)|(\\bdo\\b)|(?=\\s+(defp|defmacrop)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136548175971,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636738682979,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6858 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s(\\\\\\\\)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628113490019,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6764 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?>is_(?>atom|binary|bitstring|boolean|float|function|integer|list|map|nil|number|pid|port|record|reference|tuple|exception)|abs|bit_size|byte_size|div|elem|hd|length|map_size|node|rem|round|tl|trunc|tuple_size)\\b"),
      scope: vec![
        Scope {
            a: 52645084895838208,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }