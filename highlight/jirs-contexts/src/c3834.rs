
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
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 47288521951477942,
            b: 16044073672507392,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3835 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=#|@end|;)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 46445982501109760,
            b: 0,
        },
        Scope {
            a: 47288629324480569,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3836 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\w+(?=:)"),
      scope: vec![
        Scope {
            a: 59392130630616146,
            b: 16044073672507392,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((:))\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630616146,
            b: 16044073672507392,
        },
    ]),(2, vec![
        Scope {
            a: 47288620749357113,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629324480569,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3837 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3896 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(,)\\s+(\\.\\.\\.)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620724846592,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628128366649,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3838 }),
    ]),
      with_prototype: None
    }),
]
} }