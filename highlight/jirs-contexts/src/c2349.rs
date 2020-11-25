
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
        index: 2376,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcatch\\b"),
      scope: vec![
        Scope {
            a: 52636636706177426,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2249 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bfinally\\b"),
      scope: vec![
        Scope {
            a: 52636636706177433,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\btry\\b"),
      scope: vec![
        Scope {
            a: 52636636706177290,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2319 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bassert\\b"),
      scope: vec![
        Scope {
            a: 52636636701196844,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2252 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bbreak\\b"),
      scope: vec![
        Scope {
            a: 52636636701196708,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcontinue\\b"),
      scope: vec![
        Scope {
            a: 52636636701196754,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\breturn\\b"),
      scope: vec![
        Scope {
            a: 52636636701196639,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bthrow\\b"),
      scope: vec![
        Scope {
            a: 52636636701196709,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bif\\b"),
      scope: vec![
        Scope {
            a: 52636636711616746,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\belse\\b"),
      scope: vec![
        Scope {
            a: 52636636711616777,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bswitch\\b"),
      scope: vec![
        Scope {
            a: 52636636711616905,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcase\\b"),
      scope: vec![
        Scope {
            a: 52636636711616906,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdefault\\b"),
      scope: vec![
        Scope {
            a: 52636636711617297,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdo\\b"),
      scope: vec![
        Scope {
            a: 52636636708537119,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bfor\\b"),
      scope: vec![
        Scope {
            a: 52636636708536544,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2319 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwhile\\b"),
      scope: vec![
        Scope {
            a: 52636636708536556,
            b: 11258999068426240,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }