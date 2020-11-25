
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
      regex: Regex::new("(?<=\\})"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7067 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(((mutable)\\s[\\p{L}]+)|[\\p{L}0-9\'`<>^._]*)\\s*((?<!:):(?!:))\\s*"),
      scope: vec![],
      captures: Some(vec![(3, vec![
        Scope {
            a: 52636271616458752,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52638333207642112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7048 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7070 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7071 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7084 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7066 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7073 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7072 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7064 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7063 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7076 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7065 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7087 })),
]
} }