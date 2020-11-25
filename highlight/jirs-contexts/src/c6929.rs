
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
      regex: Regex::new("(?x)\n\\\\g(?:\n  <( ((?>-[1-9]\\d*|\\d+) | [a-zA-Z_][a-zA-Z_\\d]{,31}) | \\g<-1>?([^\\[\\\\(){}|^$.?*+\\n]+) )> | \'\\g<1>\' |\n  (<([^\\[\\\\(){}|^$.?*+\\n]+*)>? | \'\\g<-1>\'?) )"),
      scope: vec![
        Scope {
            a: 52636787156385836,
            b: 27866022694354944,
        },
    ],
      captures: Some(vec![(1, vec![]),(2, vec![
        Scope {
            a: 49259087435857964,
            b: 27866022694354944,
        },
    ]),(3, vec![
        Scope {
            a: 50103314797494316,
            b: 27866022694354944,
        },
    ]),(4, vec![
        Scope {
            a: 50103314797494316,
            b: 27866022694354944,
        },
    ]),(5, vec![
        Scope {
            a: 50103314797494316,
            b: 27866022694354944,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n\\(\\?(?:\n  ([+-]?\\d+|R|(?:P>|&)[a-zA-Z_][a-zA-Z_\\d]{,31})\\) |\n  (?:P>|&)([^\\[\\\\(){}|^$.?*+\\n]+)\\)? |\n  (R|P>|&)\\)? )"),
      scope: vec![
        Scope {
            a: 52636787156385836,
            b: 27866022694354944,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087435857964,
            b: 27866022694354944,
        },
    ]),(2, vec![
        Scope {
            a: 50103314797494316,
            b: 27866022694354944,
        },
    ]),(3, vec![
        Scope {
            a: 50103314797494316,
            b: 27866022694354944,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }