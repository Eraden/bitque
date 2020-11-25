
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9091 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9093 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9102 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9106 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9112 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9113 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9114 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9126 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9157 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9159 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9094 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(filter)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119699982,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8925 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(counter)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119765518,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8937 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(counters)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119765518,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8966 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(symbols)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119765518,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8969 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(format)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255117602830,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8927 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(local)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255117602830,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8929 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(matrix3d|scale3d|matrix|scale)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119831054,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8931 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(translate(3d)?)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119831054,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8933 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(translate[XY])(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119831054,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8935 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(rotate[XYZ]?|skew[XY])(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119831054,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8938 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(skew)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119831054,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8940 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(translateZ|perspective)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119831054,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8942 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(rotate3d)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119831054,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8944 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(scale[XYZ])(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119831054,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8946 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(cubic-bezier)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119896590,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8949 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(steps)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119896590,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8951 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(rect)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119962126,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8953 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(inset)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119962126,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8955 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(circle|ellipse)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119962126,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8957 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(polygon)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255119962126,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8960 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(toggle)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255120027662,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8962 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(repeat)(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255120093198,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8964 }),
    ]),
      with_prototype: None
    }),
]
} }