
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 990 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 992 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 999 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1003 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1007 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1008 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1009 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1020 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1040 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 1042 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 993 })),
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
        ContextReference::Direct(ContextId { index: 851 }),
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
        ContextReference::Direct(ContextId { index: 863 }),
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
        ContextReference::Direct(ContextId { index: 892 }),
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
        ContextReference::Direct(ContextId { index: 895 }),
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
        ContextReference::Direct(ContextId { index: 853 }),
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
        ContextReference::Direct(ContextId { index: 855 }),
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
        ContextReference::Direct(ContextId { index: 857 }),
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
        ContextReference::Direct(ContextId { index: 859 }),
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
        ContextReference::Direct(ContextId { index: 861 }),
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
        ContextReference::Direct(ContextId { index: 864 }),
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
        ContextReference::Direct(ContextId { index: 866 }),
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
        ContextReference::Direct(ContextId { index: 868 }),
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
        ContextReference::Direct(ContextId { index: 870 }),
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
        ContextReference::Direct(ContextId { index: 872 }),
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
        ContextReference::Direct(ContextId { index: 875 }),
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
        ContextReference::Direct(ContextId { index: 877 }),
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
        ContextReference::Direct(ContextId { index: 879 }),
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
        ContextReference::Direct(ContextId { index: 881 }),
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
        ContextReference::Direct(ContextId { index: 883 }),
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
        ContextReference::Direct(ContextId { index: 886 }),
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
        ContextReference::Direct(ContextId { index: 888 }),
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
        ContextReference::Direct(ContextId { index: 890 }),
    ]),
      with_prototype: None
    }),
]
} }