
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
        index: 399,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s*(\\.)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925375345360896,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788226933110,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s*(<)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925375345360896,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444612403527680,
            b: 0,
        },
        Scope {
            a: 47288629329985718,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 318 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b))(\\?)?((\\[)(,*)(\\]))?(?:\\s*(\\*))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576463249408,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576488022027,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46444990360649728,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288521961570486,
            b: 3096224743817216,
        },
    ]),(5, vec![
        Scope {
            a: 47288620721831936,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288521961570475,
            b: 3096224743817216,
        },
    ]),(7, vec![
        Scope {
            a: 52636628123516939,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 320 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))(\\?)?((\\[)(,*)(\\]))?(?:\\s*(\\*))?)(?:\\s+((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)))?\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925375345360896,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576488022027,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46444990360649728,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288521961570486,
            b: 3096224743817216,
        },
    ]),(5, vec![
        Scope {
            a: 47288620721831936,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288521961570475,
            b: 3096224743817216,
        },
    ]),(7, vec![
        Scope {
            a: 52636628123516939,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 49259087292727296,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }