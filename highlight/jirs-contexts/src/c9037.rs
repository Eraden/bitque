
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
        a: 46445510051889152,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46445510051889152,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\s*([)]))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9101 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9100 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9128 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9131 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9159 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9142 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9149 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9140 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9139 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)(\\s+(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*))*\\b(?!:)"),
      scope: vec![
        Scope {
            a: 55451949097418752,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9123 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9155 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9146 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9099 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?m:$))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }