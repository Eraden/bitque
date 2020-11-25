
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
      regex: Regex::new("^/\\* =(\\s*.*?)\\s*= \\*/(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510878523162624,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445273858506975,
            b: 31806672368304128,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("/\\*"),
      scope: vec![
        Scope {
            a: 47288629323038833,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7457 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*/(?!\\*)"),
      scope: vec![
        Scope {
            a: 50103314684444785,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^// =(\\s*.*?)\\s*=\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711042769009,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445273858506936,
            b: 31806672368304128,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("//"),
      scope: vec![
        Scope {
            a: 47288629323038833,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7458 }),
    ]),
      with_prototype: None
    }),
]
} }