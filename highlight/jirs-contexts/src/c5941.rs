
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
      regex: Regex::new("(?x)\n^(:)                          # opening delimiter\n(!)?                          # bang symbol (unset attribute)\n([A-Za-z0-9_][A-Za-z0-9_-]*)  # attribute name\n(!)?                          # bang symbol (unset attribute)\n(:)                           # closing delimiter\n(?:\\s+|(?=(?m:$)))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629412431389,
            b: 51228806538592256,
        },
    ]),(2, vec![
        Scope {
            a: 47288629412431291,
            b: 23643898043695104,
        },
    ]),(3, vec![
        Scope {
            a: 61925246524981332,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629412431291,
            b: 23643898043695104,
        },
    ]),(5, vec![
        Scope {
            a: 47288629412431389,
            b: 48132581794775040,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5916 }),
    ]),
      with_prototype: None
    }),
]
} }