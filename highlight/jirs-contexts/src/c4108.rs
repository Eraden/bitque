
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
      regex: Regex::new("\\b(?xi:\n  TRUE | FALSE | NULL |\n  __CLASS__ | __DIR__ | __FILE__ | __FUNCTION__ | __LINE__ | __METHOD__ | __NAMESPACE__\n)\\b"),
      scope: vec![
        Scope {
            a: 59955110641008640,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\\\?\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\\\\\\\?(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b\\\\)*\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3999 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\\\?[\\p{L}_])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4000 }),
    ]),
      with_prototype: None
    }),
]
} }