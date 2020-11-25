
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
        a: 46444217269813248,
        b: 0,
    },
    Scope {
        a: 55451949100498944,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 46444217269813248,
        b: 0,
    },
    Scope {
        a: 55451949100498944,
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
      regex: Regex::new("^\\.(?=\\s*(?m:$))"),
      scope: vec![
        Scope {
            a: 47288689522245693,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.{4,}"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.{3}"),
      scope: vec![
        Scope {
            a: 59955136434012161,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("~{3,}"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("~{1,2}"),
      scope: vec![
        Scope {
            a: 59955136434012161,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([@^])[<|>*]+"),
      scope: vec![
        Scope {
            a: 59955136434012161,
            b: 17169973579350016,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629338046646,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([@^])0?[.#]+(?![_\\d])"),
      scope: vec![
        Scope {
            a: 59955136434012374,
            b: 17169973579350016,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629338046646,
            b: 17169973579350016,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 4387 })),
]
} }