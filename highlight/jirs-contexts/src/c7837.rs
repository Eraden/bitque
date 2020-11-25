
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
      regex: Regex::new("(?x)\n(?:(?<=\\.)|\\b)\n([A-Z][$\\w]*)\\s*(\\.)\n([_$a-zA-Z][$\\w]*)\\s*\n(\\(\\s*\\))"),
      scope: vec![
        Scope {
            a: 46444883017402454,
            b: 12103423998558208,
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
            a: 49258881136394240,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 46443865103795470,
            b: 49822922011508736,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?:(?<=\\.)|\\b)\n([A-Z][$\\w]*)\\s*(\\.)\n([_$a-zA-Z][$\\w]*)\\s*\n(?=\\()"),
      scope: vec![
        Scope {
            a: 46444883017402445,
            b: 12103423998558208,
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
            a: 49258881136394240,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?<=\\.)\n([_$a-zA-Z][$\\w]*)\\s*\n(\\(\\s*\\))"),
      scope: vec![
        Scope {
            a: 46444882996627542,
            b: 12103423998558208,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258881136394240,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865103795470,
            b: 49822922011508736,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(?<=\\.)\n([_$a-zA-Z][$\\w]*)\\s*\n(?=\\()"),
      scope: vec![
        Scope {
            a: 46444882996627533,
            b: 12103423998558208,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258881136394240,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }