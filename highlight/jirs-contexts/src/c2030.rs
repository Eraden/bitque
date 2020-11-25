
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
      regex: Regex::new("(?x)^\\s*\n  (?: # zero or more modifiers\n    (?:\n      (public|private|protected)|(final)|(native|synchronized|abstract|threadsafe|transient)\n    )\n    \\s+\n  )?\n  \\s*\n  ([A-Z](?:[a-zA-Z0-9_])+) # constructor/class name\n  \\s*\n  (?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439060275233,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439071547425,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414439038320673,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630615404,
            b: 9288674231451648,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1996 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)^\\s*\n  (?:\n    (?: # or modifier and optional type\n      (?:(?:\\b(public|private|protected)|(static)|(final)|(native|synchronized|abstract|threadsafe|transient))\\b\\s+)+\\s* # modifier\n      (?:\\b\n        (void\\b)\n        |\n        ((?:boolean|byte|char|short|int|float|long|double)\\b) # primitive\n        |\n        ( # or class type\n          (?:\\w+\\.)*[A-Z]\\w+\\b # Class name\n          (?:<(?:[\\w, ]*)>)? # optional Generic type\n          (?:\\[\\s*\\])* # zero or more square brackets (array)\n        )\n      )?\n    )\n    |\n    (?:\\b # or type by itself\n      (def\\b)\n      |\n      (void\\b)\n      |\n      ((?:boolean|byte|char|short|int|float|long|double)\\b) # primitive\n      |\n      ( # or class type\n        (?:\\w+\\.)*[A-Z]\\w+\\b # Class name\n        (?:<(?:[\\w, ]*)>)? # optional generics info\n        (?:\\[\\s*\\])* # zero or more square brackets (array)\n      )\n    )\n  )\n  \\s*\n  (\\w+) # method name\n  \\s*\n  (?=\\() # opening parens"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439060275233,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439055228961,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414439071547425,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414439038320673,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 48414576502375144,
            b: 9288674231451648,
        },
    ]),(6, vec![
        Scope {
            a: 48414576502375145,
            b: 9288674231451648,
        },
    ]),(7, vec![
        Scope {
            a: 48414576502374603,
            b: 9288674231451648,
        },
    ]),(8, vec![
        Scope {
            a: 48414576502374704,
            b: 9288674231451648,
        },
    ]),(9, vec![
        Scope {
            a: 48414576502375144,
            b: 9288674231451648,
        },
    ]),(10, vec![
        Scope {
            a: 48414576502375145,
            b: 9288674231451648,
        },
    ]),(11, vec![
        Scope {
            a: 48414576502374603,
            b: 9288674231451648,
        },
    ]),(12, vec![
        Scope {
            a: 59392130630615073,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 1997 }),
    ]),
      with_prototype: None
    }),
]
} }