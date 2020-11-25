
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
      regex: Regex::new("(?x:\n  # builtin data types\n  # http://erlang.org/doc/reference_manual/typespec.html#the-erlang-type-language\n  any|arity|atom|binary|bitstring|boolean|byte|char|float|fun|function|\n  identifier|integer|iodata|iolist|list|map|maybe_improper_list|mfa|module|nil|\n  no_return|node|none|non_neg_integer|neg_integer|pos_integer|nonempty_list|\n  nonempty_maybe_improper_list|nonempty_improper_list|\n  nonempty_maybe_improper_list|nonempty_string|\n  number|pid|port|record|reference|string|term|timeout|tuple|\n\n  # erlang library\n  # http://erlang.org/doc/man/erlang.html#data-types\n  dist_handle|ext_binary|iovec|message_queue_data|nif_resource|\n  deprecated_time_unit|timeout|timestamp|time_unit\n)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 46445879419469824,
            b: 0,
        },
        Scope {
            a: 61925375345950720,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[a-z][_A-Za-z\\d@]*"),
      scope: vec![
        Scope {
            a: 46445879419469824,
            b: 0,
        },
        Scope {
            a: 48414576463839232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\'"),
      scope: vec![
        Scope {
            a: 47288629349318838,
            b: 5629499534213120,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1499 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1580 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1518 })),
]
} }