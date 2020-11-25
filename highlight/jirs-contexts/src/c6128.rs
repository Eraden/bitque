
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("="),
      scope: vec![
        Scope {
            a: 52636628111130709,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:(?:v)?start|align|absolute)\\b"),
      scope: vec![
        Scope {
            a: 49258876878192725,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:(?:prog|no)bits|private|public|common|stack|code|text|data|bss|rdata|info)\\b"),
      scope: vec![
        Scope {
            a: 48414439063158869,
            b: 24488322973827072,
        },
        Scope {
            a: 49258876878192725,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:mixed|zerofill|no_dead_strip|live_support|strip_static_syms)\\b"),
      scope: vec![
        Scope {
            a: 48414439063158869,
            b: 24488322973827072,
        },
        Scope {
            a: 49258876878194239,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:(?:no)?(?:alloc|exec|write)|tls)\\b"),
      scope: vec![
        Scope {
            a: 48414439063158869,
            b: 24488322973827072,
        },
        Scope {
            a: 49258876878194343,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:(?:v)?follows)\\b"),
      scope: vec![
        Scope {
            a: 49258876878192725,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6049 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:class|overlay)\\b"),
      scope: vec![
        Scope {
            a: 49258876878192725,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6050 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6066 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6067 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6068 })),
]
} }