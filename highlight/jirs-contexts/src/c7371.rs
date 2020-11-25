
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
      regex: Regex::new("(?xi:^  \\s*  \\#  \\s*  (define|undef|if|ifdef|elif|ifndef|else|endif)  \\b  .*  (?m:$))"),
      scope: vec![
        Scope {
            a: 52636636705392837,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  \\#  \\s*  (include|import)  \\b  )"),
      scope: vec![
        Scope {
            a: 52636636705392837,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7373 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  \\#  \\s*  (error|warning)  \\b  )"),
      scope: vec![
        Scope {
            a: 52636636705392837,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7372 }),
    ]),
      with_prototype: None
    }),
]
} }