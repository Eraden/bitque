
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 7854 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:any|bool|boolean|mixed|number|string|void)\\b"),
      scope: vec![
        Scope {
            a: 59955136561678640,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[_$a-zA-Z][$\\w]*"),
      scope: vec![
        Scope {
            a: 49259087446147115,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?"),
      scope: vec![
        Scope {
            a: 52636628252886041,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }