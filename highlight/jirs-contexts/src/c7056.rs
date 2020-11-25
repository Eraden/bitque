
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
      regex: Regex::new("(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956395,
            b: 29554872554618880,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\(?m:$)[ \\t]*"),
      scope: vec![
        Scope {
            a: 47288620734022685,
            b: 29554872554618880,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\([\\\\\'\'ntbr]|u[a-fA-F0-9]{4}|u[a-fA-F0-9]{8})"),
      scope: vec![
        Scope {
            a: 59955200844431601,
            b: 29554872554618880,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\(?![\\\\\'\'ntbr]|u[a-fA-F0-9]{4}|u[a-fA-F0-9]{8})."),
      scope: vec![
        Scope {
            a: 50112033452982469,
            b: 29554872554618880,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7083 })),
]
} }