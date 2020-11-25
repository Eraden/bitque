
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
      regex: Regex::new("}"),
      scope: vec![
        Scope {
            a: 47288521951477818,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4107 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(as)\\b"),
      scope: vec![
        Scope {
            a: 52636787086196794,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(insteadof)\\b"),
      scope: vec![
        Scope {
            a: 52636787086262330,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4103 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("::"),
      scope: vec![
        Scope {
            a: 47288788251050042,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\\\?(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\\\)*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3986 }),
    ]),
      with_prototype: None
    }),
]
} }