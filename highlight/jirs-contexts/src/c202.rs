
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 199 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 206 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 215 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 210 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 208 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 200 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 209 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 201 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 219 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 198 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)exit\\b"),
      scope: vec![
        Scope {
            a: 52636636702375945,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(goto|call)\\b(?:\\s*(:)?(?:(eof)|(\\w+)))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636702375945,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620721700864,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636636701196639,
            b: 2533274790395904,
        },
    ]),(4, vec![
        Scope {
            a: 46444882986336256,
            b: 0,
        },
        Scope {
            a: 49258881134166016,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(if)\\s+(?:(not)\\s+)?(exist|defined|errorlevel|cmdextversion)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636711616521,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628114800649,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636787013124096,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)(?:if|else)\\b"),
      scope: vec![
        Scope {
            a: 52636636711616521,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i)for\\b"),
      scope: vec![
        Scope {
            a: 52636636706439177,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }