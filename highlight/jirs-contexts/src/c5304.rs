
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 281818574094336,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281818574094336,
        b: 0,
    },
],
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5301 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5302 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 47288629360394422,
            b: 22517998136852480,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5199 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</)((?:([\\p{L}_][\\p{L}\\p{N}_.-]*)(:))?([\\p{L}_][\\p{L}\\p{N}_.-]*))(>)"),
      scope: vec![
        Scope {
            a: 281818574094336,
            b: 0,
        },
        Scope {
            a: 46444230155960320,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 22517998136852480,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122448,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629324153003,
            b: 22517998136852480,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5303 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5305 })),
]
} }