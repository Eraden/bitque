
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
      regex: Regex::new("(?=\\)|(?m:$))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 74 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?i:ByRef|ByVal)\\s+)?([a-zA-Z]\\w*|\\[(?:(?!%>|\\]).)*(?:\\]|(\\n|(?=%>))))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439034978308,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208772,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?![,)])\\S)+"),
      scope: vec![
        Scope {
            a: 50103314665439236,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }