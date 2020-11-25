
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
        a: 845017635618816,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845017635618816,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\#.*"),
      scope: vec![
        Scope {
            a: 51510711032872960,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(nameserver|domain|search|sortlist|options)"),
      scope: vec![
        Scope {
            a: 52636636688678912,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(debug|ndots|timeout|attempts|rotate|no-check-names|inet6|ip6-bytestring|ip6-dotint|no-ip6-dotint|edns0|single-request|single-request-reopen|no-tld-query|use-vc|no-reload)"),
      scope: vec![
        Scope {
            a: 59392130619015168,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }