
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 1054 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[)\\]}]"),
      scope: vec![
        Scope {
            a: 50103314654625792,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\("),
      scope: vec![
        Scope {
            a: 47288521948004534,
            b: 4222124650659840,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1067 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288521961570486,
            b: 4222124650659840,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1046 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("#?{"),
      scope: vec![
        Scope {
            a: 47288521962160310,
            b: 4222124650659840,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1047 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\""),
      scope: vec![
        Scope {
            a: 47288629323956406,
            b: 4222124650659840,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1071 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("#"),
      scope: vec![
        Scope {
            a: 52636628128301071,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1061 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'|`|~|@"),
      scope: vec![
        Scope {
            a: 52636628128301071,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 1063 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\\\\S[^\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\#\']*"),
      scope: vec![
        Scope {
            a: 59955200832503808,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:nil|true|false)(?=[\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\])"),
      scope: vec![
        Scope {
            a: 59955110638190592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(:):?[^:\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\][^\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\]*"),
      scope: vec![
        Scope {
            a: 59955136419266575,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323300879,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[-+]?0\\d+N?(?=[\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\])"),
      scope: vec![
        Scope {
            a: 50104723403898880,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([-+]?)\\d+(N?)(?=[\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\#\'])"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 4222124650659840,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 4222124650659840,
        },
    ]),(2, vec![
        Scope {
            a: 48414576476553231,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([-+]?)(0[Xx])\\h+(N?)(?=[\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\#\'])"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 4222124650659840,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 4222124650659840,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325070764,
            b: 4222124650659840,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553231,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([-+]?)((?:[2-9]|[1-9]\\d+)[Rr])[0-9A-Za-z]+(?=[\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\#\'])"),
      scope: vec![
        Scope {
            a: 59955089176461537,
            b: 4222124650659840,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 4222124650659840,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325070764,
            b: 4222124650659840,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([-+]?)\\d+(/)\\d+(?=[\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\#\'])"),
      scope: vec![
        Scope {
            a: 59955089198350554,
            b: 4222124650659840,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 4222124650659840,
        },
    ]),(2, vec![
        Scope {
            a: 47288620757090319,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([-+]?)\\d+(?:(?:(\\.)\\d+(?:[eE][-+]?\\d+)?|(?:[eE][-+]?\\d+))(M)?|(M))(?=[\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\#\'])"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 4222124650659840,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629325070884,
            b: 4222124650659840,
        },
    ]),(2, vec![
        Scope {
            a: 47288620735397903,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48414576476553231,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 48414576476553231,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[^\\s,;\\(\\)\\[\\]{}\\\"`~@\\^\\\\]+"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }