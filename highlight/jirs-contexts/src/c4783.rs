
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
      regex: Regex::new("^(\\s*)((\\.\\.)\\s+raw(::)) html"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 46445965322092544,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629350629446,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620737429574,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4775 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\s*)((\\.\\.)\\s+[A-z][A-z0-9\\-_]+(::))\\s*"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 46444337564483654,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629350629446,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620737429574,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4776 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\s*).*?((::))"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 114280120449368064,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629330575430,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4777 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("::"),
      scope: vec![
        Scope {
            a: 46449173662662656,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\*\\*)[^*]+(\\*\\*)"),
      scope: vec![
        Scope {
            a: 114281679522496512,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629368193094,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629368193094,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\*)\\w[^*]+\\w(\\*)"),
      scope: vec![
        Scope {
            a: 114282585760595968,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629368193094,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629368193094,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\.\\.)\\s+(_)([\\w\\s]+)(:)\\s+(.*)"),
      scope: vec![
        Scope {
            a: 46443487132778800,
            b: 19703248369745920,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629312815174,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629323956294,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 55451536781411254,
            b: 19703248369745920,
        },
    ]),(4, vec![
        Scope {
            a: 47288620737429574,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 114280588597985350,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\|)[^|]+(\\|_{0,2})"),
      scope: vec![
        Scope {
            a: 114280588619087942,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629333917766,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(\\w+)(_)\\b"),
      scope: vec![
        Scope {
            a: 46443487132778496,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 55451536781411254,
            b: 19703248369745920,
        },
    ]),(2, vec![
        Scope {
            a: 47288629312815174,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(`)([\\w\\s]+)(`_)"),
      scope: vec![
        Scope {
            a: 46443487132778496,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629312815174,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 55451536781411254,
            b: 19703248369745920,
        },
    ]),(3, vec![
        Scope {
            a: 47288629312815174,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(`)([\\w\\s]+)\\s+(<)(.*?)(>)(`_)"),
      scope: vec![
        Scope {
            a: 46443487170723910,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629312815174,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 55451536781411254,
            b: 19703248369745920,
        },
    ]),(3, vec![
        Scope {
            a: 47288629399650374,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 114280588597985350,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629399650374,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629312815174,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\.\\.)\\s+((\\[)(((#?)[^]]*?)|\\*)(\\]))\\s+(.*)"),
      scope: vec![
        Scope {
            a: 46443487178457392,
            b: 19703248369745920,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629312815174,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136464093211,
            b: 19703248369745920,
        },
    ]),(3, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(8, vec![
        Scope {
            a: 55451536836722758,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\[)[0-9]+(\\]))(_)"),
      scope: vec![
        Scope {
            a: 46443487178457302,
            b: 19703248369745920,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136464093211,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\[#)[A-z0-9_]*(\\]))(_)"),
      scope: vec![
        Scope {
            a: 46443487178458441,
            b: 19703248369745920,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136464093211,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\[)\\*(\\]))(_)"),
      scope: vec![
        Scope {
            a: 46443487178457673,
            b: 380835944137228288,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136464093211,
            b: 19703248369745920,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\.\\.)\\s+((\\[)[A-z][A-z0-9]*(\\]))(_)\\s+(.*)"),
      scope: vec![
        Scope {
            a: 46443487179833648,
            b: 19703248369745920,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629312815174,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136465469467,
            b: 19703248369745920,
        },
    ]),(3, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 55451536838099014,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\[)[A-z][A-z0-9_-]*(\\]))(_)"),
      scope: vec![
        Scope {
            a: 46443487179833414,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59955136465469467,
            b: 19703248369745920,
        },
    ]),(2, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288629325004870,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("``"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629330575430,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4778 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(`)[^`]+(`)(?!_)"),
      scope: vec![
        Scope {
            a: 114279806932877382,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629399781446,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629399781446,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(:)[A-z][A-z0-9  =\\s\\t_]*(:)"),
      scope: vec![
        Scope {
            a: 59392130632122438,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629351153734,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629351153734,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\+-[+-]+"),
      scope: vec![
        Scope {
            a: 114279806961647686,
            b: 0,
        },
    ],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629360525382,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\+=[+=]+"),
      scope: vec![
        Scope {
            a: 114279806961647686,
            b: 0,
        },
    ],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629360525382,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(^(=|-|~|`|#|\"|\\^|\\+|\\*){3,}(?m:$)){1,1}?"),
      scope: vec![
        Scope {
            a: 114281636572823552,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629353709638,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(\\s*)(\\.\\.)"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 47288629323038790,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 4779 }),
    ]),
      with_prototype: None
    }),
]
} }