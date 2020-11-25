
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
      regex: Regex::new("(?i)(?<=[\\s\\[\\(\\{:,])(?:true|false|null|none)(?=[\\s\\)\\]\\}\\,])"),
      scope: vec![
        Scope {
            a: 59955110644809728,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=[\\s\\[\\(\\{:,]|\\.\\.|\\*\\*)[0-9]+(?:\\.[0-9]+)?(?=[\\s\\)\\]\\}\\,]|\\.\\.|\\*\\*)"),
      scope: vec![
        Scope {
            a: 59955089169973248,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }