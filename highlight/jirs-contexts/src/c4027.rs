
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
  uses_backrefs: true,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: true,
      regex: Regex::new("^\\s*(\\1)\\b(?:(;)(\\s*(?m:$)\\n?))?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288521949642923,
            b: 16325548649218048,
        },
        Scope {
            a: 52636628172996666,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288689463132218,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 46448241653972992,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(HTML)\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628172996666,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4028 }),
        ContextReference::Direct(ContextId { index: 4122 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4039 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(\'HTML\')\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46444208676208640,
            b: 0,
        },
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628173127738,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4047 }),
        ContextReference::Direct(ContextId { index: 2107 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4048 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(XML)\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628172996666,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4049 }),
        ContextReference::Direct(ContextId { index: 4126 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4050 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(\'XML\')\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46444208681123840,
            b: 0,
        },
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628173127738,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4051 }),
        ContextReference::Direct(ContextId { index: 5819 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4052 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(SQL)\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628172996666,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4053 }),
        ContextReference::Direct(ContextId { index: 4125 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4029 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(\'SQL\')\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46444208680402944,
            b: 0,
        },
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628173127738,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4030 }),
        ContextReference::Direct(ContextId { index: 5102 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4031 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(JAVASCRIPT)\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628172996666,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4032 }),
        ContextReference::Direct(ContextId { index: 4123 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4033 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(\'JAVASCRIPT\')\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46444208678699008,
            b: 0,
        },
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628173127738,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4034 }),
        ContextReference::Direct(ContextId { index: 7848 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4035 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<<<)\\s*(JSON)\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628172996666,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4036 }),
        ContextReference::Direct(ContextId { index: 4124 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4037 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(<<<)\\s*(\'JSON\')\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46444208678371328,
            b: 0,
        },
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628173127738,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4038 }),
        ContextReference::Direct(ContextId { index: 2200 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4040 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(CSS)\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628172996666,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4041 }),
        ContextReference::Direct(ContextId { index: 4121 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4042 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(\'CSS\')\\s*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 46444208676798464,
            b: 0,
        },
        Scope {
            a: 47288521949642934,
            b: 16325548649218048,
        },
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628173127738,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4043 }),
        ContextReference::Direct(ContextId { index: 1017 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 4044 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)"),
      scope: vec![
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628172996666,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4045 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("<<<\\s*(\'(\\b[\\p{L}_][\\p{L}\\p{N}_]*\\b)\')"),
      scope: vec![
        Scope {
            a: 47288629323956282,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628173127738,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4046 }),
    ]),
      with_prototype: None
    }),
]
} }