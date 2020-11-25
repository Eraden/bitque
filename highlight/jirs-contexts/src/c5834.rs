
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
      regex: Regex::new("(?x)\n(?:\n  (?:\n    ([\\p{L}_][\\p{L}\\p{N}_.-]*)           # 1: valid namespace\n    |\n    ([^:=/<>\\s]+)              # 2: invalid namespace\n  )(:)\n)?                             # namespace is optional\n(?:\n  ([\\p{L}_][\\p{L}\\p{N}_.-]*)             # 3: valid localname\n  |\n  ([^=/<>\\s]+?)                # 4: invalid localname\n)(?=[=<>\\s]|[/?]>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392186477183350,
            b: 22517998136852480,
        },
    ]),(2, vec![
        Scope {
            a: 50103314746441808,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392186477183056,
            b: 0,
        },
        Scope {
            a: 47288620745621584,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392186477184391,
            b: 22517998136852480,
        },
    ]),(5, vec![
        Scope {
            a: 50103314746441808,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5835 }),
    ]),
      with_prototype: None
    }),
]
} }