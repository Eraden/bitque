
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
        index: 6241,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521948004534,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6231 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bDEFINED\\b"),
      scope: vec![
        Scope {
            a: 49258876853946047,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bSTREQUAL\\b"),
      scope: vec![
        Scope {
            a: 49258876853946048,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bNOT\\b"),
      scope: vec![
        Scope {
            a: 52636628114802369,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bAND\\b"),
      scope: vec![
        Scope {
            a: 52636628114802370,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bOR\\b"),
      scope: vec![
        Scope {
            a: 52636628114802371,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bCOMMAND\\b"),
      scope: vec![
        Scope {
            a: 49258876853946052,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bPOLICY\\b"),
      scope: vec![
        Scope {
            a: 49258876853946053,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bTARGET\\b"),
      scope: vec![
        Scope {
            a: 49258876853946054,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bTEST\\b"),
      scope: vec![
        Scope {
            a: 49258876853946055,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bEXISTS\\b"),
      scope: vec![
        Scope {
            a: 49258876853946056,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bIS_NEWER_THAN\\b"),
      scope: vec![
        Scope {
            a: 49258876853946057,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bIS_DIRECTORY\\b"),
      scope: vec![
        Scope {
            a: 49258876853946058,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bIS_SYMLINK\\b"),
      scope: vec![
        Scope {
            a: 49258876853946059,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bIS_ABSOLUTE\\b"),
      scope: vec![
        Scope {
            a: 49258876853946060,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(VERSION_|STR)?LESS\\b"),
      scope: vec![
        Scope {
            a: 49258876853946061,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(VERSION_|STR)?GREATER\\b"),
      scope: vec![
        Scope {
            a: 49258876853946062,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(VERSION_|STR)?EQUAL\\b"),
      scope: vec![
        Scope {
            a: 49258876853946063,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(VERSION_|STR)?LESS_EQUAL\\b"),
      scope: vec![
        Scope {
            a: 49258876853946064,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(VERSION_|STR)?GREATER_EQUAL\\b"),
      scope: vec![
        Scope {
            a: 49258876853946065,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bMATCHES\\b"),
      scope: vec![
        Scope {
            a: 49258876853946066,
            b: 24769797950537728,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6178 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6200 })),
]
} }