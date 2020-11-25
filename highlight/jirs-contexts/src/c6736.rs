
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
      regex: Regex::new("^\\s*(?i:(onbuild)\\s+)?(?i:add|arg|env|expose|healthcheck|label|run|shell|stopsignal|user|volume|workdir)\\s"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636636695035904,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636787102253153,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6733 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?i:maintainer)\\s"),
      scope: vec![
        Scope {
            a: 52636636695035904,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6733 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*((?i:(onbuild)\\s+)?(?i:copy))(?:\\s+--from=(\\S+))?\\s"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636695035904,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787102253153,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49267346514116608,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6733 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?i:(onbuild)\\s+)?(?i:cmd|entrypoint)\\s"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636628105101312,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636787102253153,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6733 }),
    ]),
      with_prototype: None
    }),
]
} }