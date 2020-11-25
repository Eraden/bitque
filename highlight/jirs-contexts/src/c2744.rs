
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
        a: 281668250238976,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 281668250238976,
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
      regex: Regex::new("@Comment"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323038765,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2730 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)String)\\s*(\\{)\\s*([a-zA-Z]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787068174381,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300909,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521992503478,
            b: 12666373951979520,
        },
    ]),(4, vec![
        Scope {
            a: 49259087294955520,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2731 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)String)\\s*(\\()\\s*([a-zA-Z]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787068174381,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300909,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521992503478,
            b: 12666373951979520,
        },
    ]),(4, vec![
        Scope {
            a: 49259087294955520,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2732 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)[a-zA-Z]+)\\s*(\\{)\\s*([^\\s,]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787068436525,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300909,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521992634550,
            b: 12666373951979520,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632450902,
            b: 12666373951979520,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2733 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)[a-zA-Z]+)\\s*(\\()\\s*([^\\s,]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787068436525,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300909,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288521992634550,
            b: 12666373951979520,
        },
    ]),(4, vec![
        Scope {
            a: 59392130632450902,
            b: 12666373951979520,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2735 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[^@\\n]"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2737 }),
    ]),
      with_prototype: None
    }),
]
} }