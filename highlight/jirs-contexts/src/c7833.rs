
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
      regex: Regex::new("(?x)\n(?<!\\?)(?<!\\?\\s)(?=(?>\n  ((\')((?>[^\'\\\\]|\\\\.)*)(\'))|\n  ((\")((?>[^\"\\\\]|\\\\.)*)(\"))|\n)\\s*:)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7777 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!\\.|\\?|\\?\\s)([_$a-zA-Z][$\\w]*)\\s*(:)"),
      scope: vec![
        Scope {
            a: 59955136461406503,
            b: 12103423998558208,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451949119635499,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620737429547,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }