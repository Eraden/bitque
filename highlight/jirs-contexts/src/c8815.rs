
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
        a: 845009045684224,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845009045684224,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("/\\*(?!/)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8810 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("//.*(?m:$)"),
      scope: vec![
        Scope {
            a: 51510711028613256,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(import)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787041304712,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8811 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[A-Z]\\w*\\b"),
      scope: vec![
        Scope {
            a: 61925366763618304,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(((^|\\{)\\s*)|\\b)on[A-Z]\\w*\\b"),
      scope: vec![
        Scope {
            a: 61925366763618304,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:^|\\{)\\s*(id)\\s*\\:\\s*([^;\\s]+)\\b"),
      scope: vec![
        Scope {
            a: 46445574484393984,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787021447168,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439032487936,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?:(default|readonly)\\s+)?(property)\\s+(?:(alias)|([\\w\\<\\>]+))\\s+(\\w+)"),
      scope: vec![
        Scope {
            a: 46454014095130624,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787021447168,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636787021447168,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636787021447168,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576471441408,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59392186477183112,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(signal)\\s+(\\w+)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787021447168,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925255094468608,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8812 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:\\b|\\s+)(?:(true|false|null|undefined)|(var|void)|(on|as|enum|connect|break|case|catch|continue|debugger|default|delete|do|else|finally|for|if|in|instanceof|new|return|switch|this|throw|try|typeof|while|with))\\b"),
      scope: vec![
        Scope {
            a: 46444174325055488,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955110646120448,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576471441408,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636636697591808,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(function)\\s+([\\w_]+)\\s*(?=\\()"),
      scope: vec![
        Scope {
            a: 46444131375382528,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576471441408,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630617519,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\w_]+\\s*(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255094468608,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:^|\\{|;)\\s*[a-z][\\w\\.]*\\s*(?=\\:)"),
      scope: vec![
        Scope {
            a: 59392186477183112,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=\\.)\\b\\w*"),
      scope: vec![
        Scope {
            a: 59392186477183112,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([a-z_]\\w*)\\b"),
      scope: vec![
        Scope {
            a: 49258876838608896,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 7848 })),
]
} }