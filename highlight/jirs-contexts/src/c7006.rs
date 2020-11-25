
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
      regex: Regex::new("^((?i:From|To|Cc|Bcc|Date))(:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445780653244992,
            b: 29273397577908224,
        },
        Scope {
            a: 59392130642149480,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757876985,
            b: 29273397577908224,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7005 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(Subject)(:)[ \\t]*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445780653244992,
            b: 29273397577908224,
        },
        Scope {
            a: 59392130642149480,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757876985,
            b: 29273397577908224,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6975 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(?x:(\n  ARC-Seal|\n  (?:ARC-)?(?:Authentication-Results)|\n  ([xX]-)?(?:\\w+-)*(?:DomainKey|DKIM|Message)-Signature|\n  Received-SPF\n))(:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46445780653244992,
            b: 29273397577908224,
        },
        Scope {
            a: 61925255092371456,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259830328164352,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288620757876985,
            b: 29273397577908224,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7005 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7001 })),
]
} }