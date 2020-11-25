
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
      regex: Regex::new("(\\*)(?i:trace-output|terminal-io|suppress-series-warnings|standard-output|standard-input|readtable|read-suppress|read-eval|read-default-float-format|read-base|random-state|query-io|print-right-margin|print-readably|print-radix|print-pretty|print-pprint-dispatch|print-miser-width|print-lines|print-level|print-length|print-gensym|print-escape|print-circle|print-case|print-base|print-array|package|modules|macroexpand-hook|load-verbose|load-truename|load-print|load-pathname|gensym-counter|features|evalhook|error-output|default-pathname-defaults|debugger-hook|debug-io|compile-verbose|compile-print|compile-file-truename|compile-file-pathname|break-on-warnings|break-on-signals|applyhook)(\\*)"),
      scope: vec![
        Scope {
            a: 49259061525217280,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514614,
            b: 12947848928690176,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514603,
            b: 12947848928690176,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\*)(\\S*)(\\*)"),
      scope: vec![
        Scope {
            a: 49259087310356526,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629322514614,
            b: 12947848928690176,
        },
    ]),(3, vec![
        Scope {
            a: 47288629322514603,
            b: 12947848928690176,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }