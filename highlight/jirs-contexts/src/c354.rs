
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
        a: 51510878542233611,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 51510878542233611,
        b: 0,
    },
],
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<)((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122550,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 242 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(</)((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))(>)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629324153014,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632122539,
            b: 3096224743817216,
        },
    ]),(3, vec![
        Scope {
            a: 47288629324153003,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(///)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323039124,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?!///)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 341 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[\\w\\s]+|."),
      scope: vec![
        Scope {
            a: 283210144219136,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }