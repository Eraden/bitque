
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
      regex: Regex::new("(:)(active|checked|default|disabled|empty|enabled|first-child|first-of-type|first|fullscreen|focus|hover|indeterminate|in-range|invalid|last-child|last-of-type|left|link|only-child|only-of-type|optional|out-of-range|read-only|read-write|required|right|root|scope|target|valid|visited)\\b"),
      scope: vec![
        Scope {
            a: 59392186477183482,
            b: 40813871623045120,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 701436475200438417,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(:?:)(before|after)\\b"),
      scope: vec![
        Scope {
            a: 59392186477183518,
            b: 40813871623045120,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 701436475200438417,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(::)(first-letter|first-number|selection)\\b"),
      scope: vec![
        Scope {
            a: 59392186477183518,
            b: 40813871623045120,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 701436475200438417,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((:)dir)\\s*(?:(\\()(ltr|rtl)?(\\)))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186477183518,
            b: 40813871623045120,
        },
    ]),(2, vec![
        Scope {
            a: 701436475200438417,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 701436475203126157,
            b: 40813871623045120,
        },
    ]),(4, vec![
        Scope {
            a: 59955110646710272,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 701436475203125419,
            b: 40813871623045120,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((:)lang)\\s*(?:(\\()(\\w+(-\\w+)?)?(\\)))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186477183518,
            b: 40813871623045120,
        },
    ]),(2, vec![
        Scope {
            a: 701436475200438417,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 701436475203126157,
            b: 40813871623045120,
        },
    ]),(4, vec![
        Scope {
            a: 59955110646710272,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 701436475203125419,
            b: 40813871623045120,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9225 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9224 })),
]
} }