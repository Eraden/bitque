
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
  prototype: Some(
    ContextId {
        index: 5074,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{\\{|\\}\\}"),
      scope: vec![
        Scope {
            a: 59955200847315016,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)                      # Spec from http://doc.rust-lang.org/std/fmt/\n\\{\n  (\\d+|(?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))?\n  (\n    :                     # format_spec delimiter\n    (.?[<>^])?            # [[fill]align]\n    [+-]?                 # [sign]\n    \\#?                   # [\'#\']\n    0?                    # [0]\n    (\\d\\$?)?              # [width]\n    (\\.(\\d\\$?|\\*)?)?      # [\'.\' precision]\n    (\\?|(?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))?  # [type]\n  )?\n\\}"),
      scope: vec![
        Scope {
            a: 59955136434012232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }