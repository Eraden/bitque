
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
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\""),
      scope: vec![
        Scope {
            a: 47288629323956395,
            b: 5629499534213120,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\\\)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1539 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  (~)\n  (?: (\\-?\\*) | (\\-?\\d+) )?  # F - field width\n  (?:\n    (\\.) (?: (\\*) | (\\d+) )? # P - precision\n    (?: (\\.) (.) )?          # Pad - padding character\n  )?\n  ([lt]{0,2})                # Mod - control modifier\n  ([~cfegswpWPBX#bx\\+ni])    # C - control sequence\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136434012180,
            b: 0,
        },
        Scope {
            a: 47288629338046484,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136434012796,
            b: 179299646064033792,
        },
    ]),(3, vec![
        Scope {
            a: 59955136434012796,
            b: 60517205892136960,
        },
    ]),(4, vec![
        Scope {
            a: 59955136434012798,
            b: 5629499534213120,
        },
        Scope {
            a: 47288620748111892,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59955136434012798,
            b: 179299646064033792,
        },
    ]),(6, vec![
        Scope {
            a: 59955136434012798,
            b: 60517205892136960,
        },
    ]),(7, vec![
        Scope {
            a: 59955136434012352,
            b: 5629499534213120,
        },
        Scope {
            a: 47288620748111892,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 59955136434012799,
            b: 67554080309903360,
        },
    ]),(9, vec![
        Scope {
            a: 59955136434012350,
            b: 48695256870289408,
        },
    ]),(10, vec![
        Scope {
            a: 59955136434012350,
            b: 67554080309903360,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }