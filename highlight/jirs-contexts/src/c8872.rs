
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
      regex: Regex::new("\\)"),
      scope: vec![
        Scope {
            a: 47288629318582443,
            b: 3940649673949184,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9100 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9101 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n(\n    (-webkit-|-o-)?\n    ((min|max)-)?\n    (-moz-)?\n    (\n        ((device-)?(height|width|aspect-ratio|pixel-ratio))|\n        (color(-index)?)|monochrome|resolution\n    )\n)|grid|scan|orientation\n\\s*(?=[:)])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 61925375377670637,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 61925375377735694,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 61925375377735694,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8873 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(portrait|landscape|progressive|interlace)"),
      scope: vec![
        Scope {
            a: 61925409737015310,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\d+)(/)(\\d+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089163288576,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628119191566,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955089163288576,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9128 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9148 })),
]
} }