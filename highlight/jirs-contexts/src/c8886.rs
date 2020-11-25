
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
      regex: Regex::new("(@)(each)\\b"),
      scope: vec![
        Scope {
            a: 52636636701196429,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8842 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(for|while)\\b"),
      scope: vec![
        Scope {
            a: 52636636701196429,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8854 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(if|else if|else)\\b"),
      scope: vec![
        Scope {
            a: 52636636701196638,
            b: 39687971716202496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8864 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(debug|warn|error)\\b"),
      scope: vec![
        Scope {
            a: 52636636720398477,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8865 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(at-root)\\b"),
      scope: vec![
        Scope {
            a: 52636636720400819,
            b: 39687971716202496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(extend)\\b"),
      scope: vec![
        Scope {
            a: 52636636720400779,
            b: 39687971716202496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8866 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)(mixin|function))\\s+([\\w-]+)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398477,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392130630615181,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8867 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)(include))\\s+(([a-z]+)(\\.))?([\\w-]+)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49258881142816768,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636636720398477,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 59392130643525773,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288620745621645,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8844 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(@)(return)\\b"),
      scope: vec![
        Scope {
            a: 52636636720398687,
            b: 39687971716202496,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8846 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*((@)(use|forward)\\b)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720399099,
            b: 39687971716202496,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8847 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)(?:-(?:webkit|moz|o)-)?(charset|namespace|font-face)\\b)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398350,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300878,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8850 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*((@)counter-style\\b)\\s+(?:(?i:\\b(decimal|none)\\b)|((?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)))?\\s*(?=\\{|(?m:$))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398822,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300878,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 50103314687524878,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392186487472142,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8852 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\s*@custom-media\\b)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8853 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)document)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398824,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300878,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8856 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*((@)import\\b)\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398775,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300878,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8857 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*((@)(-webkit-|-moz-|-o-)?keyframes)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398826,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300878,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8858 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*((@)media)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398829,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300878,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8859 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*((@)namespace)\\s+((?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*)(?!(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[^[:ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))|\\())?"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398710,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300878,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392186487537678,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8860 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s*((@)page)\\s*(?:(:)(first|left|right))?\\s*"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398830,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300878,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629324873742,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392186486751246,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8861 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)supports)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398831,
            b: 3940649673949184,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323300878,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8862 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((@)content)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636720398991,
            b: 39687971716202496,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323301005,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8863 }),
    ]),
      with_prototype: None
    }),
]
} }