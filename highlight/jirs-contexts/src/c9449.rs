
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
      regex: Regex::new("(?=;|(?m:$)|\\})"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9578 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(if)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636711616662,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46445243847803030,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9450 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=\\))\\s*\\/(?![\\/*])(?=(?:[^\\/\\\\\\[]|\\\\.|\\[([^\\]\\\\]|\\\\.)+\\])+\\/([gimsuy]+|(?![\\/\\*])|(?=\\/\\*))(?!\\s*[a-zA-Z0-9_$]))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 42221246506598400,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9451 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9664 })),
]
} }