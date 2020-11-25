
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 8269 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8271 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8279 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8284 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8288 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8289 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8290 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8309 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8328 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8330 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 8272 })),
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
        ContextReference::Direct(ContextId { index: 8101 }),
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
        ContextReference::Direct(ContextId { index: 8113 }),
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
        ContextReference::Direct(ContextId { index: 8142 }),
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
        ContextReference::Direct(ContextId { index: 8145 }),
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
        ContextReference::Direct(ContextId { index: 8103 }),
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
        ContextReference::Direct(ContextId { index: 8105 }),
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
        ContextReference::Direct(ContextId { index: 8107 }),
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
        ContextReference::Direct(ContextId { index: 8109 }),
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
        ContextReference::Direct(ContextId { index: 8111 }),
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
        ContextReference::Direct(ContextId { index: 8114 }),
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
        ContextReference::Direct(ContextId { index: 8116 }),
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
        ContextReference::Direct(ContextId { index: 8118 }),
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
        ContextReference::Direct(ContextId { index: 8120 }),
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
        ContextReference::Direct(ContextId { index: 8122 }),
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
        ContextReference::Direct(ContextId { index: 8125 }),
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
        ContextReference::Direct(ContextId { index: 8127 }),
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
        ContextReference::Direct(ContextId { index: 8129 }),
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
        ContextReference::Direct(ContextId { index: 8131 }),
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
        ContextReference::Direct(ContextId { index: 8133 }),
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
        ContextReference::Direct(ContextId { index: 8136 }),
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
        ContextReference::Direct(ContextId { index: 8138 }),
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
        ContextReference::Direct(ContextId { index: 8140 }),
    ]),
      with_prototype: None
    }),
]
} }