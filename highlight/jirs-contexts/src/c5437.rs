
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
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\d*)(<<-)\\s*(\')([\\p{L}_][\\p{L}\\p{N}_]*)(\')"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 405042809314213888,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130981,
            b: 20829148276588544,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956406,
            b: 20829148276588544,
        },
    ]),(4, vec![
        Scope {
            a: 52636636782788682,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629323956395,
            b: 20829148276588544,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5424 }),
        ContextReference::Direct(ContextId { index: 5427 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\d*)(<<-)\\s*(\")([\\p{L}_][\\p{L}\\p{N}_]*)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 405042809314213888,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130981,
            b: 20829148276588544,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956406,
            b: 20829148276588544,
        },
    ]),(4, vec![
        Scope {
            a: 52636636782788682,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629323956395,
            b: 20829148276588544,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5424 }),
        ContextReference::Direct(ContextId { index: 5427 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\d*)(<<-)\\s*(\\\\)([\\p{L}_][\\p{L}\\p{N}_]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 405042809314213888,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130981,
            b: 20829148276588544,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956298,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636636782788682,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5424 }),
        ContextReference::Direct(ContextId { index: 5427 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\d*)(<<-)\\s*([\\p{L}_][\\p{L}\\p{N}_]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 405042809314213888,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130981,
            b: 20829148276588544,
        },
    ]),(3, vec![
        Scope {
            a: 52636636782788682,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5423 }),
        ContextReference::Direct(ContextId { index: 5427 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\d*)(<<)\\s*(\')([\\p{L}_][\\p{L}\\p{N}_]*)(\')"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 405042809314213888,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130981,
            b: 20829148276588544,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956406,
            b: 20829148276588544,
        },
    ]),(4, vec![
        Scope {
            a: 52636636782788682,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629323956395,
            b: 20829148276588544,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5426 }),
        ContextReference::Direct(ContextId { index: 5427 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\d*)(<<)\\s*(\")([\\p{L}_][\\p{L}\\p{N}_]*)(\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 405042809314213888,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130981,
            b: 20829148276588544,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956406,
            b: 20829148276588544,
        },
    ]),(4, vec![
        Scope {
            a: 52636636782788682,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629323956395,
            b: 20829148276588544,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5426 }),
        ContextReference::Direct(ContextId { index: 5427 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\d*)(<<)\\s*(\\\\)([\\p{L}_][\\p{L}\\p{N}_]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 405042809314213888,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130981,
            b: 20829148276588544,
        },
    ]),(3, vec![
        Scope {
            a: 47288629323956298,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636636782788682,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5426 }),
        ContextReference::Direct(ContextId { index: 5427 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\d*)(<<)\\s*([\\p{L}_][\\p{L}\\p{N}_]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089176461530,
            b: 405042809314213888,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130981,
            b: 20829148276588544,
        },
    ]),(3, vec![
        Scope {
            a: 52636636782788682,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5422 }),
        ContextReference::Direct(ContextId { index: 5427 }),
    ]),
      with_prototype: None
    }),
]
} }