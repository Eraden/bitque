
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
      regex: Regex::new("\\b(CORE)(::)(?=\\b(?x:\n  # control keywords\n  default|else|elsif|given|if|unless|when|break|caller|continue|die|\n  do|dump|exit|goto|last|next|redo|return|wait|for|foreach|until|while|\n  # declaration keywords\n  package|require|use|no|sub|format|local|my|our|state|\n  # word operators\n  and|or|xor|as|cmp|eq|gt|ge|lt|le|ne|not|x|\n  # quoted like functions (are handled like keywords)\n  m|q|qq|qr|qw|qx|s|tr|y\n)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259727246131200,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788251050045,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:local|my|our|state)\\b(?!::)"),
      scope: vec![
        Scope {
            a: 48414576473997373,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdefault\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636711617297,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\belse\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636711616777,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\belsif\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636711616920,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bgiven\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636711617766,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bif\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636711616746,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bunless\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636711617767,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwhen\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636711616908,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bbreak\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701196708,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcaller\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701197544,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bcontinue\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701196754,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdie\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701197545,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdo\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701196510,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bdump\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701197546,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bexit\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701197547,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bgoto\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701196710,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\blast\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701197548,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4312 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bnext\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701197549,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4312 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bredo\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701197550,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4312 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\breturn\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701196639,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwait\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636701197551,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bfor\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636708536544,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bforeach\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636708536735,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\buntil\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636708536592,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bwhile\\b(?!::)"),
      scope: vec![
        Scope {
            a: 52636636708536556,
            b: 17169973579350016,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4291 }),
    ]),
      with_prototype: None
    }),
]
} }