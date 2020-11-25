
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 8394 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\t ]([0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}|[0-9a-f:]+)(\\/[0-9]{2})?(?=[\\t ;])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55460650708500480,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955089320378494,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\t ](=?[0-9][0-9\\.]*[bBkKmMgGtTsShHdD]?)(?=[\\t ;])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955089170628608,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\t ](on|off|true|false)(?=[\\t ;])"),
      scope: vec![
        Scope {
            a: 59955110645465088,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\t ](kqueue|rtsig|epoll|\\/dev\\/poll|select|poll|eventport|max|all|default_server|default|main|crit|error|debug|warn|notice|last)(?=[\\t ;])"),
      scope: vec![
        Scope {
            a: 59955110645465088,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\.*\\ |\\~\\*|\\~|\\!\\~\\*|\\!\\~"),
      scope: vec![
        Scope {
            a: 55450759398817792,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\^.*?\\$"),
      scope: vec![
        Scope {
            a: 55450759398817792,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8388 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8389 }),
    ]),
      with_prototype: None
    }),
]
} }