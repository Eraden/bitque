
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
      regex: Regex::new("\\b(item|container|(computer|disk|trash)-object|disk|folder|((alias|application|document|internet location) )?file|clipping|package)s?\\b"),
      scope: vec![
        Scope {
            a: 61925366773383490,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((Finder|desktop|information|preferences|clipping) )windows?\\b"),
      scope: vec![
        Scope {
            a: 61925366773383491,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(preferences|(icon|column|list) view options|(label|column|alias list)s?)\\b"),
      scope: vec![
        Scope {
            a: 61925366773383492,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(copy|find|sort|clean up|eject|empty( trash)|erase|reveal|update)\\b"),
      scope: vec![
        Scope {
            a: 61925255104233794,
            b: 2251799813685248,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(insertion location|product version|startup disk|desktop|trash|home|computer container|finder preferences)\\b"),
      scope: vec![
        Scope {
            a: 61925409723056136,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(visible)\\b"),
      scope: vec![
        Scope {
            a: 61925246514298888,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }