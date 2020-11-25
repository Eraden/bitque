
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
        index: 6125,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:ideal|jumps|p[345]86|end)\\b"),
      scope: vec![
        Scope {
            a: 50104723507642453,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:bits|use(?:16|32|64)|org|uppercase|safeseh|osabi)\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(default)(?:\\s+(rel|abs|(?:no)?bnd))?\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ]),(2, vec![
        Scope {
            a: 61925409743962197,
            b: 24488322973827072,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:section|segment)\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6128 }),
        ContextReference::Direct(ContextId { index: 6127 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\bgroup\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6127 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:__SECT__|__NASMDEFSEG|_?_GLOBAL_OFFSET_TABLE_)\\b"),
      scope: vec![
        Scope {
            a: 61925409743962197,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:absolute|common)\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\b(?:subsections_via_symbols|no_dead_strip)\\b"),
      scope: vec![
        Scope {
            a: 61925255125141055,
            b: 23925746682560512,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)import\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6014 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)export\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6023 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)global\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6025 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)extern\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6026 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)library\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6027 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)module\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6028 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\bcpu\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6016 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)\\bfloat\\b"),
      scope: vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6017 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(\\[)\\s*(warning)\\b"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6018 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i)(\\[)\\s*(map)\\b"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 61925255125139541,
            b: 24488322973827072,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6020 }),
    ]),
      with_prototype: None
    }),
]
} }