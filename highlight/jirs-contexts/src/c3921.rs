
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 3896 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*"),
      scope: vec![
        Scope {
            a: 52636628102479872,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\n  (struct|union|enum)\n  \\s+\n  (?=\n    \\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\n    (\\s+\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b(?!\\s*[{=;])|\\s*\\*+)\n  )\n)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576466264064,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3919 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b(struct|union|enum)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3898 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b(break|case|continue|default|do|else|for|goto|if|_Pragma|return|switch|while)\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\s)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3919 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\\s*(\\()(?=[^\\)]+\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258881137311744,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865082232832,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 16044073672507392,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3831 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3832 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3976 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\W)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }