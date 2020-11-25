
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
  prototype: Some(
    ContextId {
        index: 5435,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 5401 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((-)[aobcdefghknoprstuvwxzGLNORS])(?=\\s)"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 47288629322580042,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 49258876882255946,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((-)(?:ef|nt|ot|eq|ne|lt|le|gt|ge))(?=\\s)"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 47288629322580042,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 49258876882255946,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(=~)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628114800714,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5348 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("==?|!=?|<|>|\\|\\||&&"),
      scope: vec![
        Scope {
            a: 52636628114800714,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }