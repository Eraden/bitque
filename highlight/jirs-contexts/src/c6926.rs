
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
      regex: Regex::new("\\[:[<>]:\\]"),
      scope: vec![
        Scope {
            a: 50104723546570796,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[::\\]"),
      scope: vec![
        Scope {
            a: 50103314741329964,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) (\\[:) (?=(\\\\\\\\ | \\[(?!:) | (?<=\\\\)[\\[\\]] | [^\\[\\]])*? :])"),
      scope: vec![
        Scope {
            a: 52636636704867512,
            b: 12385324177031168,
        },
        Scope {
            a: 47288629398732982,
            b: 12385324177031168,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6899 }),
    ]),
      with_prototype: None
    }),
]
} }