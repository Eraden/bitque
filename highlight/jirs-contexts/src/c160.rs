
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
      regex: Regex::new("\\b(artwork|application|encoder|EQ preset|item|source|visual|(EQ |browser )?window|((audio CD|device|shared|URL|file) )?track|playlist window|((audio CD|device|radio tuner|library|folder|user) )?playlist)s?\\b"),
      scope: vec![
        Scope {
            a: 61925366773514248,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(add|back track|convert|fast forward|(next|previous) track|pause|play(pause)?|refresh|resume|rewind|search|stop|update|eject|subscribe|update(Podcast|AllPodcasts)|download)\\b"),
      scope: vec![
        Scope {
            a: 61925255104364552,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(current (playlist|stream (title|URL)|track)|player state)\\b"),
      scope: vec![
        Scope {
            a: 61925409723187208,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(current (encoder|EQ preset|visual)|EQ enabled|fixed indexing|full screen|mute|player position|sound volume|visuals enabled|visual size)\\b"),
      scope: vec![
        Scope {
            a: 61925246514429960,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }