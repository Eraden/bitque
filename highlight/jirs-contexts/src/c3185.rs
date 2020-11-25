
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 844631088562176,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844631088562176,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 3170 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3197 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?x)(?:              # ignore whitespace in this regex\n  (?:[^()]*(?=[()])\\(        # start level 1                      __\n    (?:[^()]*(?=[()])\\(      # start level 2      ___/ _____ \\__/ /\n      (?:[^()]*(?=[()])\\(    # start level 3     is like snek... (by Valerie Haecky)\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n    (?:[^()]*(?=[()])\\(      # start level 2\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n    [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n  [^()]*(?=[()])\n  \\)[^()]*)?       #   end level 1\n  |[^()]*)\n:(?!=)(?x)(?:              # ignore whitespace in this regex\n  (?:[^()]*(?=[()])\\(        # start level 1                      __\n    (?:[^()]*(?=[()])\\(      # start level 2      ___/ _____ \\__/ /\n      (?:[^()]*(?=[()])\\(    # start level 3     is like snek... (by Valerie Haecky)\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      \\)[^()]*)?   #   end level 3\n      [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n    (?:[^()]*(?=[()])\\(      # start level 2\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n      (?:[^()]*(?=[()])\\(    # start level 3\n        (?:[^()]*(?=[()])\\(  # start level 4\n          [^()]*(?=[()]) #       level 4\n        \\)[^()]*)? #   end level 4\n      [^()]*(?=[()])\n      \\)[^()]*)?   #   end level 3\n    [^()]*(?=[()])\n    \\)[^()]*)?     #   end level 2\n  [^()]*(?=[()])\n  \\)[^()]*)?       #   end level 1\n  |[^()]*)\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 3176 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3199 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3173 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(endef)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 50103314682151839,
            b: 13510798882111488,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }