
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
      regex: Regex::new("end(?=(?m:$)|[[^\\n\\S]\\n\\);&\\|<>])"),
      scope: vec![
        Scope {
            a: 46444882999640170,
            b: 0,
        },
        Scope {
            a: 52636636711616618,
            b: 0,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(for)(\\s+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444882999640170,
            b: 0,
        },
        Scope {
            a: 52636636711616618,
            b: 0,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444882992693248,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7123 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7231 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("in(?=\\s)"),
      scope: vec![
        Scope {
            a: 46444882999640170,
            b: 0,
        },
        Scope {
            a: 52636636711616618,
            b: 0,
        },
        Scope {
            a: 46444217286852714,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7126 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\n)|(;)|(?=[)#])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444882998067390,
            b: 29836347531329536,
        },
    ]),(2, vec![
        Scope {
            a: 46444882998067390,
            b: 29836347531329536,
        },
        Scope {
            a: 52636628111196266,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7105 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\S+?"),
      scope: vec![
        Scope {
            a: 50103314676711530,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7252 })),
]
} }