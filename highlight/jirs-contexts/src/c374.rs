
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
      regex: Regex::new("\\b(if)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636711616746,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 355 }),
        ContextReference::Direct(ContextId { index: 365 }),
        ContextReference::Direct(ContextId { index: 366 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(switch)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196681,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 410 }),
        ContextReference::Direct(ContextId { index: 366 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(for)\\s*(\\()(?=(?:(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)|var)\\s(?!=))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636708536544,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 46443865079218176,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 360 }),
        ContextReference::Direct(ContextId { index: 375 }),
        ContextReference::Direct(ContextId { index: 375 }),
        ContextReference::Direct(ContextId { index: 361 }),
        ContextReference::Direct(ContextId { index: 427 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(for)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636708536544,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 46443865079218176,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 360 }),
        ContextReference::Direct(ContextId { index: 375 }),
        ContextReference::Direct(ContextId { index: 375 }),
        ContextReference::Direct(ContextId { index: 376 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(foreach)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636708536735,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 46443865079218176,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 360 }),
        ContextReference::Direct(ContextId { index: 362 }),
        ContextReference::Direct(ContextId { index: 427 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(try)\\b"),
      scope: vec![
        Scope {
            a: 52636636714959114,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 359 }),
        ContextReference::Direct(ContextId { index: 334 }),
        ContextReference::Direct(ContextId { index: 412 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(using)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636715941899,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865079218176,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 424 }),
        ContextReference::Direct(ContextId { index: 374 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(fixed)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703424929,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 46443865079218176,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 358 }),
        ContextReference::Direct(ContextId { index: 374 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(lock)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636703424930,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 46443865079218176,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 358 }),
        ContextReference::Direct(ContextId { index: 374 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdo\\b"),
      scope: vec![
        Scope {
            a: 52636636708536542,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 353 }),
        ContextReference::Direct(ContextId { index: 430 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwhile\\b"),
      scope: vec![
        Scope {
            a: 52636636708536556,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 430 }),
        ContextReference::Direct(ContextId { index: 431 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(else|case|catch|finally)\\b"),
      scope: vec![
        Scope {
            a: 52636636689399808,
            b: 0,
        },
        Scope {
            a: 50103314681102523,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(return|yield\\s+return)\\b"),
      scope: vec![
        Scope {
            a: 52636636701196639,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 375 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(continue|break|yield\\s+break)\\b\\s*(;)?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196708,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 47288689454284811,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(throw)\\b"),
      scope: vec![
        Scope {
            a: 52636636714959269,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 375 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(goto)\\s+(case)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196710,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 52636636714434954,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 375 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(goto)\\s+((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s*(;)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636701196710,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 59955136430145547,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288689454284811,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 372 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=(?:\\b(?:async|ref)\\s+)?(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)(?:\\s*(<[^(={};<]*>)?\\s*)?(?:\\s*(?:\\[,*\\]|\\*|\\?)*)\\s+(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)\\s*(<[^(={};<]*>)?\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 267 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s*(<)(?=([^(={};]*>)(?:\\s*(?:\\[,*\\]|\\*|\\?)*)\\s+(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)\\s*(<[^(={};<]*>)?\\s*\\()"),
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
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 390 }),
        ContextReference::Direct(ContextId { index: 416 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s*(<)(?=[^(={};]*>\\()"),
      scope: vec![
        Scope {
            a: 46444882986467328,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258881134297088,
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
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 268 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))(<)"),
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
        ContextReference::Direct(ContextId { index: 269 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bconst\\b"),
      scope: vec![
        Scope {
            a: 48414439024295936,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(var|dynamic)\\s+((?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b))\\s*(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576473997323,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314654363648,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628111130799,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(var|dynamic)\\s+((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s*(=)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576473997323,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087292727296,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628111130799,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 375 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=var\\s*\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 427 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 373 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\b(ref)\\s+)?\\b(?=(?:(?:(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)\\s*(<[^(={};<]*>)?\\s*\\s*\\.\\s*)*(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)\\s*(<[^(={};<]*>)?\\s*)(?:\\s*(?:\\[,*\\]|\\*|\\?)*)\\s+(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)(?!\\s*(?:[(:])))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439024295936,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 429 }),
        ContextReference::Direct(ContextId { index: 420 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\b(ref)\\s+)?\\b(?:((?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b))|((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)))(?=(?:\\s*(?:\\[,*\\]|\\*|\\?)*)\\s+(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)(?!\\s*:))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439024295936,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576463249408,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925375345360896,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 429 }),
        ContextReference::Direct(ContextId { index: 420 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s*(:)(?!:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130642149387,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620721831936,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 395 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s+((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925375345360896,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087292727296,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 429 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\."),
      scope: vec![
        Scope {
            a: 47288788226932747,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\S)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 375 }),
    ]),
      with_prototype: None
    }),
]
} }