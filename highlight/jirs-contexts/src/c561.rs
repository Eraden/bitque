
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
      regex: Regex::new("\\bclass\\b"),
      scope: vec![
        Scope {
            a: 48414576463314944,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 563 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(struct|union|enum\\s+class|enum\\s+struct|enum|class)(?=\\s+(?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s+(?:::\\s*)?(?:\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*::\\s*)*(?:template\\s+)?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\s*[=;\\[])"),
      scope: vec![
        Scope {
            a: 48414576463314944,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstruct\\b"),
      scope: vec![
        Scope {
            a: 48414576463314944,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 574 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\benum(\\s+(class|struct))?\\b"),
      scope: vec![
        Scope {
            a: 48414576463314944,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 567 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bunion\\b"),
      scope: vec![
        Scope {
            a: 48414576463314944,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 577 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }