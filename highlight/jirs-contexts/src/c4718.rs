
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
        a: 281767034486784,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281767034486784,
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
      regex: Regex::new("^(!!!)((?m:$)|\\s.*)"),
      scope: vec![
        Scope {
            a: 46449053403447296,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629397749828,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^ *(/)\\s*\\S.*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711098802244,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521948856388,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^( *)(/)\\s*(?m:$)"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 47288521948856388,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4709 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?:((%)([\\w:]+))|(?=\\.|#))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444230155173888,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629324152900,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632122436,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4710 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(\\\\.)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444406248833024,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?==|-|~)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4713 }),
    ]),
      with_prototype: None
    }),
]
} }