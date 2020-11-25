
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
      regex: Regex::new("\\s*(override)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636691824640,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 3160 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(define)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636691824640,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3181 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(export)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636691824640,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3161 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?!(?x)(?:              # ignore whitespace in this regex\n  (?:[^()]*(?=[()])\\(        # start level 1                      __\n    (?:[^()]*(?=[()])\\(      # start level 2      ___/ _____ \\__/ /\n      (?:[^()]*(?=[()])\\(    # start level 3     is like snek... (by Valerie Haecky)\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n    (?:[^()]*(?=[()])\\(      # start level 2\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n    [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n  [^()]*(?=[()])\n  \\)[^()]*)?       #   end level 1\n  |[^()]*)\n:(?!=)(?x)(?:              # ignore whitespace in this regex\n  (?:[^()]*(?=[()])\\(        # start level 1                      __\n    (?:[^()]*(?=[()])\\(      # start level 2      ___/ _____ \\__/ /\n      (?:[^()]*(?=[()])\\(    # start level 3     is like snek... (by Valerie Haecky)\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n    (?:[^()]*(?=[()])\\(      # start level 2\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n    [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n  [^()]*(?=[()])\n  \\)[^()]*)?       #   end level 1\n  |[^()]*)\n)(?x)(?:              # ignore whitespace in this regex\n  (?:[^()]*(?=[()])\\(        # start level 1                      __\n    (?:[^()]*(?=[()])\\(      # start level 2      ___/ _____ \\__/ /\n      (?:[^()]*(?=[()])\\(    # start level 3     is like snek... (by Valerie Haecky)\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n    (?:[^()]*(?=[()])\\(      # start level 2\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n    [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n  [^()]*(?=[()])\n  \\)[^()]*)?       #   end level 1\n  |[^()]*)\n((\\?|\\+|::?)?=|!=)(?x)(?:              # ignore whitespace in this regex\n  (?:[^()]*(?=[()])\\(        # start level 1                      __\n    (?:[^()]*(?=[()])\\(      # start level 2      ___/ _____ \\__/ /\n      (?:[^()]*(?=[()])\\(    # start level 3     is like snek... (by Valerie Haecky)\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n    (?:[^()]*(?=[()])\\(      # start level 2\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n    [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n  [^()]*(?=[()])\n  \\)[^()]*)?       #   end level 1\n  |[^()]*)\n|(?x)\n  (?x)(?:              # ignore whitespace in this regex\n  (?:[^()]*(?=[()])\\(        # start level 1                      __\n    (?:[^()]*(?=[()])\\(      # start level 2      ___/ _____ \\__/ /\n      (?:[^()]*(?=[()])\\(    # start level 3     is like snek... (by Valerie Haecky)\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n    (?:[^()]*(?=[()])\\(      # start level 2\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n    [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n  [^()]*(?=[()])\n  \\)[^()]*)?       #   end level 1\n  |[^()]*)\n\n    (\\?|\\+|::?)?=\n  (?x)(?:              # ignore whitespace in this regex\n  (?:[^()]*(?=[()])\\(        # start level 1                      __\n    (?:[^()]*(?=[()])\\(      # start level 2      ___/ _____ \\__/ /\n      (?:[^()]*(?=[()])\\(    # start level 3     is like snek... (by Valerie Haecky)\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n    (?:[^()]*(?=[()])\\(      # start level 2\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n    [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n  [^()]*(?=[()])\n  \\)[^()]*)?       #   end level 1\n  |[^()]*)\n\n    :(?!=)\n  (?x)(?:              # ignore whitespace in this regex\n  (?:[^()]*(?=[()])\\(        # start level 1                      __\n    (?:[^()]*(?=[()])\\(      # start level 2      ___/ _____ \\__/ /\n      (?:[^()]*(?=[()])\\(    # start level 3     is like snek... (by Valerie Haecky)\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n    (?:[^()]*(?=[()])\\(      # start level 2\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n    [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n  [^()]*(?=[()])\n  \\)[^()]*)?       #   end level 1\n  |[^()]*)\n\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3162 }),
    ]),
      with_prototype: None
    }),
]
} }