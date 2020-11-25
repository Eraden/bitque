
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
      regex: Regex::new(":]"),
      scope: vec![
        Scope {
            a: 52636636704867512,
            b: 12385324177031168,
        },
        Scope {
            a: 47288629398732971,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\^?)((?>alnum|alpha|ascii|blank|cntrl|digit|graph|lower|print|punct|space|upper|word|xdigit))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636704867144,
            b: 12385324177031168,
        },
    ]),(2, vec![
        Scope {
            a: 59955136494698708,
            b: 12385324177031168,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".+?(?=:])"),
      scope: vec![
        Scope {
            a: 50103314796445740,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }