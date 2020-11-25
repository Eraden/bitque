
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
      regex: Regex::new("Rebase\\s+(\\b\\h{7,}\\b)(?:(..)(\\b\\h{7,}\\b))?.+(\\b\\h{7,}\\b).*"),
      scope: vec![
        Scope {
            a: 46446381930709022,
            b: 0,
        },
        Scope {
            a: 114281636569612318,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136432635925,
            b: 8444249301319680,
        },
    ]),(2, vec![
        Scope {
            a: 47288620767117333,
            b: 8444249301319680,
        },
    ]),(3, vec![
        Scope {
            a: 59955136432635925,
            b: 8444249301319680,
        },
    ]),(4, vec![
        Scope {
            a: 59955136432635925,
            b: 8444249301319680,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }