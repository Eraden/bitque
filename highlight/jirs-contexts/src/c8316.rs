
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
      regex: Regex::new("\\!\\s*important"),
      scope: vec![
        Scope {
            a: 52636787047989262,
            b: 0,
        },
    ],
      captures: Some(vec![]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8331 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8278 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8277 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8270 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8305 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8326 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8311 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8275 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8314 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8306 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8304 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8298 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8303 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8297 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(,)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620722028544,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)"),
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
]
} }