
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
      regex: Regex::new("\\\\x\\h{2}"),
      scope: vec![
        Scope {
            a: 59955200847315536,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\x(?:\\h|([^\'\"/)\\]}>])){1,2}"),
      scope: vec![
        Scope {
            a: 59955200847315536,
            b: 27866022694354944,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314669437520,
            b: 27866022694354944,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\u(?:\\h{4}|{\\h{1,6}})"),
      scope: vec![
        Scope {
            a: 59955200847315757,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\u(?:{}|{?[^\'\"/)\\]}>]{1,6}}?)"),
      scope: vec![
        Scope {
            a: 50103314669437741,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\."),
      scope: vec![
        Scope {
            a: 59955200847316028,
            b: 27866022694354944,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }