
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
      regex: Regex::new("\\bif(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636711616746,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bthen(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636711616775,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\belif(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636711616920,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bfi(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636711616683,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5381 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\belse(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636711616777,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bfor(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636708536544,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5415 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdo(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636708536542,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdone(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636708536491,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5381 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwhile(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636708536556,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\buntil(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636708536592,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcase(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636711616906,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5374 }),
        ContextReference::Direct(ContextId { index: 5379 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcontinue(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636701196754,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bbreak(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636701196708,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5381 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\besac(?![-=\\w])"),
      scope: vec![
        Scope {
            a: 52636636711616683,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }