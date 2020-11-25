
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
      regex: Regex::new("(?x)\n(pass)\n(:{1,2})\n(\\S*)\n(\\[)(?=[^\\]]*\\])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636702572995,
            b: 368169630314790912,
        },
    ]),(2, vec![
        Scope {
            a: 59955200844104131,
            b: 23643898043695104,
        },
    ]),(3, vec![
        Scope {
            a: 114280588622365123,
            b: 23643898043695104,
        },
    ]),(4, vec![
        Scope {
            a: 59955200832963011,
            b: 51228806538592256,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5929 }),
    ]),
      with_prototype: None
    }),
]
} }