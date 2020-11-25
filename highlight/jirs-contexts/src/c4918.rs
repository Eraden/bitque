
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
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bbegin\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636703293622,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bend\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636703293611,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcase\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636711616906,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\belse\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636711616746,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\belsif\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636711616920,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bif\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636711616746,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bthen\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636711616775,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bunless\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636711617767,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwhen\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636711616908,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bensure\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636706178389,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\brescue\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636706178390,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bfor\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636708536544,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\buntil\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636708536592,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwhile\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636708536556,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bbreak\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636701196708,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bnext\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636701197549,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bredo\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636701197550,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bretry\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636701197655,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\breturn\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636701196639,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\byield\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636636701197123,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:alias|alias_method)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52638212971102274,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bundef\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52638212981588034,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:and|not|or)\\b"),
      scope: vec![
        Scope {
            a: 52636628114800706,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bin\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 52636628114800706,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:BEGIN|END)\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 59392130630616306,
            b: 18577348462903296,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:defined|block_given)\\?"),
      scope: vec![
        Scope {
            a: 61925255090602050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bsuper\\b(?![?!])"),
      scope: vec![
        Scope {
            a: 61925255090602050,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4860 }),
    ]),
      with_prototype: None
    }),
]
} }