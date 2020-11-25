
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
      regex: Regex::new("\\b(?i:yes|no)\\b"),
      scope: vec![
        Scope {
            a: 59955110657196195,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:any|none)\\b"),
      scope: vec![
        Scope {
            a: 59955110653395107,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:default)\\b"),
      scope: vec![
        Scope {
            a: 59955110688653475,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10306 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10304 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10305 })),
    Pattern::Include(ContextReference::File { name: "SSH Crypto".to_string(), sub_context: Some("ssh-MACs".to_string()) }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10245 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10241 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10253 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10237 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10332 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*|\\?"),
      scope: vec![
        Scope {
            a: 52636628132429987,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b\\d+(?=[\\s,\"])"),
      scope: vec![
        Scope {
            a: 59955089173053440,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("!"),
      scope: vec![
        Scope {
            a: 52636628114800803,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(","),
      scope: vec![
        Scope {
            a: 47288620756172963,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }