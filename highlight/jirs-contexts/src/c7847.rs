
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
      regex: Regex::new("_*?[A-Z][_$\\dA-Z]*\\b"),
      scope: vec![
        Scope {
            a: 49259087305965611,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([A-Z][$\\w]*)\\s*(\\.)([_$a-zA-Z][$\\w]*)"),
      scope: vec![
        Scope {
            a: 46444371897942059,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087305310251,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628113883179,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087307276771,
            b: 12103423998558208,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.)([_$a-zA-Z][$\\w]*)\\s*(?=[\\[\\.])"),
      scope: vec![
        Scope {
            a: 49259087346401323,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087346401323,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=\\.)\\s*([_$a-zA-Z][$\\w]*)"),
      scope: vec![
        Scope {
            a: 46444371939033131,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087307276331,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[_$a-zA-Z][$\\w]*"),
      scope: vec![
        Scope {
            a: 49259087310290987,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }