
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
      regex: Regex::new("\\b(async +)?(for)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439108182078,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636708536544,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4392 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(async +)?(with)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439108182078,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636636701196525,
            b: 17451448556060672,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4630 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bexcept\\b"),
      scope: vec![
        Scope {
            a: 52636636706177426,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4394 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bif\\b"),
      scope: vec![
        Scope {
            a: 52636636711616746,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4396 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwhile\\b"),
      scope: vec![
        Scope {
            a: 52636636708536556,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4397 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(else)\\b(?:\\s*(:))?"),
      scope: vec![
        Scope {
            a: 46444268828360969,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636711616777,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 47288521951478110,
            b: 74591135116296192,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(try)\\b(?:\\s*(:))?"),
      scope: vec![
        Scope {
            a: 46444268822921482,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636706177290,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 47288521951478027,
            b: 74872610093006848,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(finally)\\b(?:\\s*(:))?"),
      scope: vec![
        Scope {
            a: 46444268822921625,
            b: 17451448556060672,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636706177433,
            b: 17451448556060672,
        },
    ]),(2, vec![
        Scope {
            a: 47288521951478027,
            b: 115123531762630656,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\belif\\b"),
      scope: vec![
        Scope {
            a: 52636636711616920,
            b: 17451448556060672,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4398 }),
    ]),
      with_prototype: None
    }),
]
} }