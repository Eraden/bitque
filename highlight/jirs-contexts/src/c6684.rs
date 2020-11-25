
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 6669 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6667 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)\\d+(,\\d+)?(\\})"),
      scope: vec![
        Scope {
            a: 55450759530545246,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629451030622,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629451030622,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[(?:\\^?\\])?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629365833822,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6663 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629318582366,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6664 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=^|\\s)(#)\\s[[a-zA-Z0-9,. \\t?!-][^\\x{00}-\\x{7F}]]*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510711032873054,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038814,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }