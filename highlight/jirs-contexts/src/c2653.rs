
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
        index: 2664,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\}"),
      scope: vec![
        Scope {
            a: 47288521951477931,
            b: 12103423998558208,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.\\.\\."),
      scope: vec![
        Scope {
            a: 52636628152877099,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2582 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?=\n  (?x:\n  (?:(?:[_$\\p{L}\\p{Nl}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))*(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))))\n  | \'(?:[^\\\\\']|\\\\.)*\'\n  | \"(?:[^\\\\\"]|\\\\.)*\"\n  | \\[ .* \\]\n)\\s*:\n  (?: (?:(?x:\n  \\s*\n  (?:async(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))(?x:(?:\\s+|(?:/\\*(?:(?:[^*]|\\*(?!/))*)\\*/))*))?\n  function(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))\n)|(?x:\n  \\s*\n  (?:async\\s*)?\n  (?:\n    (?:(?:[_$\\p{L}\\p{Nl}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))*(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))))\n    | \\( ( [^()] | \\( [^()]* \\) )* \\)\n  )\n  \\s*\n  =>\n)) )\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2563 }),
        ContextReference::Direct(ContextId { index: 2608 }),
        ContextReference::Direct(ContextId { index: 2654 }),
        ContextReference::Direct(ContextId { index: 2655 }),
        ContextReference::Direct(ContextId { index: 2650 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:(?=\n  (?: get|set|async )(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))(?!\\s*:)\n  | \\*\n  | (?x:\n  (?:(?:[_$\\p{L}\\p{Nl}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))*(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))))\n  | \'(?:[^\\\\\']|\\\\.)*\'\n  | \"(?:[^\\\\\"]|\\\\.)*\"\n  | \\[ .* \\]\n) \\s* \\(\n))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2648 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(?:[_$\\p{L}\\p{Nl}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))*(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))))(?=\\s*(?:[},]|(?m:$)|//|/\\*))"),
      scope: vec![
        Scope {
            a: 49259087310290987,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\[)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2655 }),
        ContextReference::Direct(ContextId { index: 2547 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\\"|\')"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2655 }),
        ContextReference::Direct(ContextId { index: 2643 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[-+]?(?:\\.[0-9]|0[bxo]|\\d))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2655 }),
        ContextReference::Direct(ContextId { index: 2642 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:(?:[_$\\p{L}\\p{Nl}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\})))*(?!(?:[_$\\p{L}\\p{Nl}\\p{Mn}\\p{Mc}\\p{Nd}\\p{Pc}\\x{200C}\\x{200D}]|(?:\\\\u(?:\\h{4}|\\{\\h+\\}))))))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2655 }),
        ContextReference::Direct(ContextId { index: 2524 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2544 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":"),
      scope: vec![
        Scope {
            a: 47288620737429547,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2582 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("="),
      scope: vec![
        Scope {
            a: 52636628111130667,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2582 }),
    ]),
      with_prototype: None
    }),
]
} }