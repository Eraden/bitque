
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
      regex: Regex::new("(\\[)[^\\]]+(\\])(?={@(?:link|linkcode|linkplain|tutorial))"),
      scope: vec![
        Scope {
            a: 59955136464554462,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629367997711,
            b: 51239294848729088,
        },
    ]),(2, vec![
        Scope {
            a: 47288629367997711,
            b: 48143070104911872,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("({)((@)(?:link(?:code|plain)?|tutorial))\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629367997388,
            b: 51239294848729088,
        },
    ]),(2, vec![
        Scope {
            a: 48414576475834846,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629360394440,
            b: 711005791171117056,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9757 }),
    ]),
      with_prototype: None
    }),
]
} }