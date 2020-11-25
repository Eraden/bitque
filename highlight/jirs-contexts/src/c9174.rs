
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
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629327560875,
            b: 40813871623045120,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) # multi-line regex definition mode\n\\b\n(?:0*((?:1?[0-9]{1,2})|(?:2(?:[0-4][0-9]|5[0-5])))\\s*(,)\\s*)\n(?:0*((?:1?[0-9]{1,2})|(?:2(?:[0-4][0-9]|5[0-5])))\\s*(,)\\s*)\n(?:0*((?:1?[0-9]{1,2})|(?:2(?:[0-4][0-9]|5[0-5])))\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136441680405,
            b: 40813871623045120,
        },
        Scope {
            a: 59955136441680405,
            b: 699747414872948736,
        },
    ]),(2, vec![
        Scope {
            a: 47294449027776657,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955136441680405,
            b: 40813871623045120,
        },
        Scope {
            a: 59955136441680405,
            b: 700028889849659392,
        },
    ]),(4, vec![
        Scope {
            a: 47294449027776657,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59955136441680405,
            b: 40813871623045120,
        },
        Scope {
            a: 59955136441680405,
            b: 700310364826370048,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) # multi-line regex definition mode\n\\b\n((?:[0-9]{1,2}|100)%)(,) # red\n\\s*\n((?:[0-9]{1,2}|100)%)(,) # green\n\\s*\n((?:[0-9]{1,2}|100)%)    # blue"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136441680405,
            b: 40813871623045120,
        },
        Scope {
            a: 59955136441680405,
            b: 699747414872948736,
        },
    ]),(2, vec![
        Scope {
            a: 47294449027776657,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59955136441680405,
            b: 40813871623045120,
        },
        Scope {
            a: 59955136441680405,
            b: 700028889849659392,
        },
    ]),(4, vec![
        Scope {
            a: 47294449027776657,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59955136441680405,
            b: 40813871623045120,
        },
        Scope {
            a: 59955136441680405,
            b: 700310364826370048,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) # multi-line regex definition mode\n(?:\\s*(,)\\s*((0?\\.[0-9]+)|[0-1]))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47294449027776657,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136441680405,
            b: 40813871623045120,
        },
        Scope {
            a: 59955136441680405,
            b: 700591839803080704,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9221 })),
]
} }