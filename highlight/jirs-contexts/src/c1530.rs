
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
        index: 1627,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(false|true)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 59955110657196052,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(error|exit|ok|throw)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 59955110654705869,
            b: 5629499534213120,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(badarg|badarith|badmatch|function_clause|case_clause|if_clause|try_clause|undef|badfun|badarity|timeout_value|noproc|nocatch|system_limit)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 59955110654706284,
            b: 5629499534213120,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(none|undefined)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 59955110677905428,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(off_heap|on_heap)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 59955110677970964,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((milli|micro|nano)?second|native|perf_counter|infinity)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 59955110657392660,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((milli|micro|nano)_)?seconds(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 59955110657392660,
            b: 0,
        },
        Scope {
            a: 50104723404226560,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }