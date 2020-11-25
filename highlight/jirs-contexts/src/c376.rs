
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
      regex: Regex::new("\\b(value)\\b"),
      scope: vec![
        Scope {
            a: 49259061522923520,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 372 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(let|select|equals|from|in|where|join|orderby|group|by|on|into|into|ascending|descending)\\b(?!\\s*[.,;+=&\\|\\[\\]\\)])"),
      scope: vec![
        Scope {
            a: 52636787017056267,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?\\."),
      scope: vec![
        Scope {
            a: 47288788252557323,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
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
      regex: Regex::new("->"),
      scope: vec![
        Scope {
            a: 47288788252622859,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\+|->|-|\\*|/|%|\\|\\||&&|\\||&|\\^|<<|>>|=>|<=|<|>=|>|==|!=|\\?\\?)="),
      scope: vec![
        Scope {
            a: 52636628099465216,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\+|->|-|\\*|/|%|\\|\\||&&|\\||&|\\^|<<|>>|=>|<=|<|>=|>|==|!=|\\?\\?)|(?:\\+\\+|--|-|~|!|&|\\*)"),
      scope: vec![
        Scope {
            a: 52636628099465216,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("="),
      scope: vec![
        Scope {
            a: 52636628111130635,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 377 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 363 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("@\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 379 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@\\$|\\$@)\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 378 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))(<)(?=[^(]*>\\s*\\()"),
      scope: vec![],
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
        ContextReference::Direct(ContextId { index: 271 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))(<)(?=(?:(?:(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)\\s*(<[^(={};<]*>)?\\s*\\s*\\.\\s*)*(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)\\s*(<[^(={};<]*>)?\\s*)>\\s*\\.)"),
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
        ContextReference::Direct(ContextId { index: 272 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?!(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)\\b)(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s*(\\()"),
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
            a: 46443865079218176,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 364 }),
        ContextReference::Direct(ContextId { index: 329 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s*(\\??\\[)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087292727296,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46444990360649728,
            b: 0,
        },
        Scope {
            a: 47288521961570486,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 328 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 373 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*(\\??\\[)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444990360649728,
            b: 0,
        },
        Scope {
            a: 47288521961570486,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 328 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b))\\b"),
      scope: vec![
        Scope {
            a: 48414576463249408,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while))\\b"),
      scope: vec![
        Scope {
            a: 52636787013255168,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:(global)|((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)))\\s*(::)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61926101194833920,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61926101194833920,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788251050358,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)"),
      scope: vec![
        Scope {
            a: 49259087292727296,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("@"),
      scope: vec![
        Scope {
            a: 50103314681495563,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\?"),
      scope: vec![
        Scope {
            a: 52636628123320331,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 273 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()\\s*(\\*)\\s*((?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b))\\s*(\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ]),(2, vec![
        Scope {
            a: 52636628123516939,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087292727296,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288521944400043,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\()(?=(?:(?:(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)\\s*(<[^(={};<]*>)?\\s*\\s*\\.\\s*)*(?:@(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while)|@(?:(?:bool|byte|sbyte|char|decimal|double|float|int|uint|long|ulong|short|ushort|object|string|void)\\b)|@var|@?(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_\\p{L}])(?:(?:\\\\u\\h{4}|\\\\U\\h{8})|[_0-9\\p{L}])*\\b)\\s*(<[^(={};<]*>)?\\s*)\\s*\\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 274 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\((?=(?:[^,)(]|\\([^\\)]*\\))*,)"),
      scope: vec![
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 275 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521944400054,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 277 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![
        Scope {
            a: 47288521951477942,
            b: 3096224743817216,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 278 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\}|\\)|>|\\]|,|(?:abstract|as|base|break|case|catch|checked|class|const|continue|default|delegate|do|else|enum|event|explicit|extern|finally|fixed|for|foreach|goto|if|implicit|in|interface|internal|is|lock|nameof|namespace|new|null|operator|out|override|params|private|protected|public|readonly|ref|return|sealed|sizeof|stackalloc|static|string|struct|switch|this|throw|try|typeof|unchecked|unsafe|using|virtual|volatile|while))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }