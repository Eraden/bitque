
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
      regex: Regex::new("\\b0x\\h*\\b(?=[;\\s\\}\\]\\\\])"),
      scope: vec![
        Scope {
            a: 59955089176461390,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b0b[01]*\\b(?=[;\\s\\}\\]\\\\])"),
      scope: vec![
        Scope {
            a: 59955089176461390,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b0o[0-7]*\\b(?=[;\\s\\}\\]\\\\])"),
      scope: vec![
        Scope {
            a: 59955089176461390,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[0-9]+\\.[0-9]+\\b(?=[;\\s\\}\\]\\\\])"),
      scope: vec![
        Scope {
            a: 59955089176592462,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[0-9]+\\b(?=[;\\s\\}\\]\\\\])"),
      scope: vec![
        Scope {
            a: 59955089176461390,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }