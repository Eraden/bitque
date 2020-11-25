
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9873 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9945 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\[)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9786 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[\\\'\\\"])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9787 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=[\\]\\\'\\\"])(?=\\s*[\\(\\<])"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9788 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?![_$\\p{L}])([\\p{Nd}]+)\\s*(?=:)"),
      scope: vec![
        Scope {
            a: 46446935994990743,
            b: 0,
        },
    ],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46454254623719575,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 59955089176658071,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?:([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?=:\\s*(\n  ((async\\s+)?(\n    (function\\s*[(<*]) |\n    (function\\s+) |\n    ([_$\\p{L}][_$\\p{L}\\p{N}]*\\s*=>)\n  )) |\n  ((async\\s*)?(\n    ((<\\s*(?m:$))|([\\(]\\s*([\\{\\[]\\s*)?(?m:$))) |\n    # sure shot arrow functions even if => is on new line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?\n  [(]\\s*\n  (\n    ([)]\\s*:) |                                                                                       # ():\n    ((\\.\\.\\.\\s*)?[_$\\p{L}][_$\\p{L}\\p{N}]*\\s*:)                                                                  # [(]param: | [(]...param:\n  )\n) |\n(\n  [<]\\s*[_$\\p{L}][_$\\p{L}\\p{N}]*\\s+extends\\s*[^=>]                                                              # < typeparam extends\n) |\n# arrow function possible to detect only with => on same line\n(\n  (<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?                                                                                 # typeparameters\n  \\(\\s*(([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\])|(\\.\\.\\.\\s*[_$\\p{L}]))([^()]|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\)))*)?\\)   # parameters\n  (\\s*:\\s*([^<>\\(\\)]|\\<[^<>]+\\>|\\([^\\(\\)]+\\))+)?                                                                        # return type\n  \\s*=>                                                                                               # arrow operator\n)\n  ))\n)))"),
      scope: vec![
        Scope {
            a: 46446935994990743,
            b: 0,
        },
    ],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46454254623719575,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 59392130630615191,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:[_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?=:)"),
      scope: vec![
        Scope {
            a: 46446935994990743,
            b: 0,
        },
    ],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46454254623719575,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.\\.\\."),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636628152877207,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9789 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([_$\\p{L}][_$\\p{L}\\p{N}]*)\\s*(?=,|\\}|(?m:$))"),
      scope: vec![
        Scope {
            a: 46446935994990743,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49259087310291095,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=[_$\\p{L}][_$\\p{L}\\p{N}]*\\s*=)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9790 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46454254623719575,
            b: 0,
        },
        Scope {
            a: 47288620737429655,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9791 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9958 })),
]
} }