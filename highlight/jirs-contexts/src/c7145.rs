
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
      regex: Regex::new("(?=[[^\\n\\S]\\n\\);&\\|<>])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\%)[0-9]+(?=(?m:$)|[[^\\n\\S]\\n\\);&\\|<>])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444882997281130,
            b: 624874903564189696,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629404827754,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\%)(self)(?=(?m:$)|[[^\\n\\S]\\n\\);&\\|<>])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444882997281130,
            b: 625158818089271296,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629405483114,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259061529149440,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\%)(last)(?=(?m:$)|[[^\\n\\S]\\n\\);&\\|<>])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444882997281130,
            b: 625161334940106752,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629405483114,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259061529149440,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\%"),
      scope: vec![
        Scope {
            a: 46444217286852714,
            b: 0,
        },
        Scope {
            a: 47288629405483114,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7146 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:[+-]?[0-9]+\\.?[0-9]*|[+-]?[0-9]*\\.?[0-9]+)(?=(?m:$)|[[^\\n\\S]\\n\\);&\\|<>])"),
      scope: vec![
        Scope {
            a: 46444882997281130,
            b: 60236100282613760,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
        Scope {
            a: 59955089169317888,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?![\\s($])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7147 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?!\\s)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7149 }),
    ]),
      with_prototype: None
    }),
]
} }