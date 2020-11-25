
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
      regex: Regex::new("\\(\\)"),
      scope: vec![
        Scope {
            a: 49258876859383860,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3511 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3517 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(~)([a-z][a-zA-Z0-9\'_]*)(\\s*:\\s*)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629379989556,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122721,
            b: 14636698788954112,
        },
    ]),(3, vec![
        Scope {
            a: 47288620744245300,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3498 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\?)([a-z][a-zA-Z0-9_]*)"),
      scope: vec![
        Scope {
            a: 49258876907356212,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629379858484,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122721,
            b: 295267473907777536,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\?)(\\()([a-z_][a-zA-Z0-9\'_]*)\\s*(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629379858484,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629379858484,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632122721,
            b: 295267473907777536,
        },
    ]),(4, vec![
        Scope {
            a: 47288620791889972,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3499 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()(?=(~[a-z][a-zA-Z0-9_]*:|(\"(\\\\\"|[^\"])*\")|[^\\(\\)~\"])+(?<!:)(:>|:(?![:=])))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288522004103220,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3500 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3510 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3503 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629335883828,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3504 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629349777460,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3505 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3518 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[a-z_][a-zA-Z0-9\'_]*"),
      scope: vec![
        Scope {
            a: 49258876842016768,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }