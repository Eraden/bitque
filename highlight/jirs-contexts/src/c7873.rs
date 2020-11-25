
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
      regex: Regex::new("\\s*\\b(macro)\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636696477696,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087299805635,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*\\b(block)\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636696477696,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087299805407,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*\\b(filter)\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636696477696,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087299805705,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*\\b(is)\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636696477696,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087299806602,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=\\{\\%-|\\{\\%)\\s*\\b([a-zA-Z_][a-zA-Z0-9_]*)\\b(?!\\s*[,=])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636696477696,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(and|else|if|in|import|not|or|recursive|with(out)?\\s+context)\\b"),
      scope: vec![
        Scope {
            a: 52636636696477696,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b([Tt]rue|[Ff]alse|[Nn]one)\\b"),
      scope: vec![
        Scope {
            a: 59955110645006336,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(loop|super|self|varargs|kwargs)\\b"),
      scope: vec![
        Scope {
            a: 49259061530001408,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[a-zA-Z_][a-zA-Z0-9_]*"),
      scope: vec![
        Scope {
            a: 49259087299805184,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\+|\\-|\\*\\*|\\*|//|/|%)"),
      scope: vec![
        Scope {
            a: 52636628119191671,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\|)([a-zA-Z_][a-zA-Z0-9_]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288762462830592,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087299805705,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\.)([a-zA-Z_][a-zA-Z0-9_]*)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288762462830592,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087299805632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288762462830592,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7859 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288762462830592,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288762462830592,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7861 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\.|:|\\||,)"),
      scope: vec![
        Scope {
            a: 47288762462830592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(==|<=|=>|<|>|!=)"),
      scope: vec![
        Scope {
            a: 52636628119257207,
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
            a: 52636628111130743,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 33495522228568064,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7862 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 33495522228568064,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7863 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("@/"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629313929398,
            b: 33495522228568064,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7864 }),
    ]),
      with_prototype: None
    }),
]
} }