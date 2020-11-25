
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
      regex: Regex::new("\\\\[bB]|\\^|\\$"),
      scope: vec![
        Scope {
            a: 52636636743532588,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\[1-9]\\d*|\\\\k<([a-zA-Z_$][\\w$]*)>"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636787067453484,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 49259087294889984,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[?+*]|\\{(\\d+,\\d+|\\d+,|,\\d+|\\d+)\\}\\??"),
      scope: vec![
        Scope {
            a: 52636628154253356,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\|"),
      scope: vec![
        Scope {
            a: 52636628154187820,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()((\\?=)|(\\?!)|(\\?<=)|(\\?<!))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582316,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629318582995,
            b: 12384898975268864,
        },
    ]),(3, vec![
        Scope {
            a: 46446476473729068,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 46446476473794604,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 46446476473860140,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 46446476473925676,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9504 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\((?:(\\?:)|(?:\\?<([a-zA-Z_$][\\w$]*)>))?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629318582316,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629318583117,
            b: 12384898975268864,
        },
    ]),(2, vec![
        Scope {
            a: 49259087294889984,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9505 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\[)(\\^)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629365833772,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628153794604,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9506 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9658 })),
]
} }