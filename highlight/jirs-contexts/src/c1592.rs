
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
      regex: Regex::new("(?x:\n  # builtin namespace\n  erlang|\n\n  # erlang otp libraries\n  # https://github.com/erlang/otp\n  asn1|common_test|compiler|crypto|debugger|dialyzer|diameter|edoc|eldap|\n  erl_(docgen|interface)|et|eunit|ftp|hipe|inets|jinterface|kernel|\n  megaco|mnesia|observer|odbc|os_mon|parsetools|public_key|reltool|\n  runtime_tools|sasl|snmp|ssh|ssl|stdlib|syntax_tools|tftp|tools|wx|xmerl\n)(?=[^[_A-Za-z\\d@]])"),
      scope: vec![
        Scope {
            a: 46445879419469824,
            b: 0,
        },
        Scope {
            a: 61926101195423744,
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
            a: 49259727243444224,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1674 })),
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
        ContextReference::Direct(ContextId { index: 1430 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1580 })),
]
} }