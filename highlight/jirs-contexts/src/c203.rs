
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
      regex: Regex::new("(?=(?m:$)\\n|[&|><)])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 201 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 219 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\")\\s*([^ ][^=]*)(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323956406,
            b: 2533274790395904,
        },
    ]),(2, vec![
        Scope {
            a: 49259087310290953,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628111130633,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 172 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([^ ][^=]*)(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087310290953,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628111130633,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 173 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 208 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 210 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([^ ][^=\\n]*)(?m:$)"),
      scope: vec![
        Scope {
            a: 49259087310290953,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 215 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s+/[aA]\\s+"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 175 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s+/[pP]\\s+"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 176 }),
    ]),
      with_prototype: None
    }),
]
} }