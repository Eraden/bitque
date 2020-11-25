
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
      regex: Regex::new("\\s*:\\s*"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288620737429649,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\n    ((min|max)-)?\n    (\n        ((device-)?(height|width|aspect-ratio))|\n        (color(-index)?)|monochrome|resolution\n    )\n)|grid|scan|orientation"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 61925375377670637,
            b: 40813871623045120,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(portrait|landscape|progressive|interlace)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925409737015441,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }