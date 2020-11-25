
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
  clear_scopes: Some(
    ClearAmount::TopN(1),
),
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("}}"),
      scope: vec![
        Scope {
            a: 47288521963733163,
            b: 8725724278030336,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s-"),
      scope: vec![
        Scope {
            a: 52636628127711936,
            b: 198439991724998656,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("-\\s"),
      scope: vec![
        Scope {
            a: 52636628127711938,
            b: 198439991724998656,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":=|="),
      scope: vec![
        Scope {
            a: 52636628111130655,
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
            a: 52636628127711588,
            b: 8725724278030336,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\.)([\\w]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788226932767,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087306883103,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)[\\w]+"),
      scope: vec![
        Scope {
            a: 49259087320973343,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514463,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[.$]"),
      scope: vec![
        Scope {
            a: 49259087320973343,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(if|else|range|template|with|end|nil|define|block)\\b"),
      scope: vec![
        Scope {
            a: 52636636690710528,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(and|call|html|index|slice|js|len|not|or|print|printf|println|urlquery|eq|ne|lt|le|gt|ge)\\b"),
      scope: vec![
        Scope {
            a: 49258881135607808,
            b: 0,
        },
        Scope {
            a: 61925255090602015,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1887 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1935 })),
]
} }