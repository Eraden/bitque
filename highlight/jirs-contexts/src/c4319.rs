
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
      regex: Regex::new("%%|\\\\."),
      scope: vec![
        Scope {
            a: 59955200847315005,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  (%)                                  # punctuation\n  (?: \\d+\\$ )?                         # index\n  (?: \\s*[-+]\\s* | [ #0] )?            # flags\n  (?: \\* (?: \\d+\\$ )? | \\d+ )?         # width\n  (?:\n    v\\d* |                             # vector flag\n    (\\.) (?: \\* (?: \\d+\\$ )? | \\d+ )?  # precision\n  )?\n  (?: hh|ll|[hjlLqtz])?                # size\n  [aAbBcdDeEfFgGinoOpsuUxX]            # control sequence\n)"),
      scope: vec![
        Scope {
            a: 59955136434012221,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629338046525,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397949,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }