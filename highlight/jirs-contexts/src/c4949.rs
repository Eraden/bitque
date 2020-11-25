
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 4914 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4872 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)\\d+(,\\d+)?(\\})"),
      scope: vec![
        Scope {
            a: 46444217270140928,
            b: 0,
        },
        Scope {
            a: 55450759423459394,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629343944770,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629343944770,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[(?:\\^?\\])?"),
      scope: vec![
        Scope {
            a: 47288629365833794,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4847 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288629318582338,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4848 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:^|\\s)(#)\\s[[a-zA-Z0-9,. \\t?!-][^\\x{00}-\\x{7F}]]*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510711032873026,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038786,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }