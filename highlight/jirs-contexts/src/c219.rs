
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
      regex: Regex::new("(%)((~([fdpnxsatz]|\\$PATH:)*)?\\d|\\*)"),
      scope: vec![
        Scope {
            a: 49258876839198720,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514441,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(%)errorlevel(%)"),
      scope: vec![
        Scope {
            a: 49259061522792448,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514614,
            b: 2533274790395904,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514603,
            b: 2533274790395904,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 216 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 217 })),
]
} }