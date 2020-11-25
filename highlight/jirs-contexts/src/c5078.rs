
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
        index: 5074,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\'(?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))\\s*(:)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130642149448,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288620725829632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'(?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b)(?!\\\')\\b"),
      scope: vec![
        Scope {
            a: 48414439113621576,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:(pub)\\s+)?\\b(mod)\\s+((?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439028293632,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576512532552,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130669019208,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5000 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))\\s*(=)\\s*(?=\\|)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130630615112,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628103462912,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5029 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:(pub)\\s+)?\\b(fn)\\s+(?=(?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))"),
      scope: vec![
        Scope {
            a: 46444131371188224,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439028293632,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576474128456,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5041 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:(pub)\\s+)?(struct)\\s+"),
      scope: vec![
        Scope {
            a: 46445102033797120,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439028293632,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576488939592,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5084 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:(pub)\\s+)?(type)\\s+((?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439028293632,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576475963464,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130632450120,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:(pub)\\s+)?(trait)\\s+((?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439028293632,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576537763912,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130694250568,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5001 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bimpl\\b"),
      scope: vec![
        Scope {
            a: 48414576552312904,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5055 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:(pub)\\s+)?(enum)\\s+((?:(?:[\\p{L}][_\\p{L}\\p{N}]*|_[_\\p{L}\\p{N}]+)\\b))\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414439028293632,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576486318152,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130642804808,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5002 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(let|const|static)\\b"),
      scope: vec![
        Scope {
            a: 48414576467247104,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*const\\b"),
      scope: vec![
        Scope {
            a: 48414576467247104,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bfn\\b"),
      scope: vec![
        Scope {
            a: 48414576474128456,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bmod\\b"),
      scope: vec![
        Scope {
            a: 48414576512532552,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bstruct\\b"),
      scope: vec![
        Scope {
            a: 48414576488939592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\bimpl\\b"),
      scope: vec![
        Scope {
            a: 48414576552312904,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\benum\\b"),
      scope: vec![
        Scope {
            a: 48414576486318152,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\btype\\b"),
      scope: vec![
        Scope {
            a: 48414576475963464,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\btrait\\b"),
      scope: vec![
        Scope {
            a: 48414576537763912,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(mut|pub|unsafe|move|ref)\\b"),
      scope: vec![
        Scope {
            a: 48414439028293632,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\*mut\\b"),
      scope: vec![
        Scope {
            a: 48414439028293632,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(crate|extern|use|where)\\b"),
      scope: vec![
        Scope {
            a: 52636787017252864,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(break|else|for|if|loop|match|while|continue)\\b"),
      scope: vec![
        Scope {
            a: 52636636693397504,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\breturn\\b"),
      scope: vec![
        Scope {
            a: 52636636693397504,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5018 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(as|in|box)\\b"),
      scope: vec![
        Scope {
            a: 52636628103462912,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(virtual|proc|alignof|become|offsetof|priv|pure|sizeof|typeof|unsized|yield|do|abstract|final|override|macro)\\b"),
      scope: vec![
        Scope {
            a: 50103314658361344,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(true|false)\\b"),
      scope: vec![
        Scope {
            a: 59955110641926144,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5087 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(macro_rules!)\\s+([\\p{L}_][\\p{L}\\p{N}_]*)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255090274304,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130648571976,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5061 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5034 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5081 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5028 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\p{Ll}_][\\p{Ll}\\p{Nd}_]*(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255090274304,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?:format|print|println|eprint|eprintln)!)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61926431911313408,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865083215872,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 20266198323167232,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5003 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b((?:write|writeln)!)\\s*(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61926431911313408,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865083215872,
            b: 0,
        },
        Scope {
            a: 47288521944400054,
            b: 20266198323167232,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5004 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[\\p{Ll}_][\\p{Ll}\\p{Nd}_]*!(?=\\s*(\\(|\\{|\\[))"),
      scope: vec![
        Scope {
            a: 61926431911313408,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Copy|Send|Sized|Sync|Drop|Fn|FnMut|FnOnce|Box|ToOwned|Clone|PartialEq|PartialOrd|Eq|Ord|AsRef|AsMut|Into|From|Default|Iterator|Extend|IntoIterator|DoubleEndedIterator|ExactSizeIterator|Option|Some|None|Result|Ok|Err|SliceConcatExt|String|ToString|Vec)\\b"),
      scope: vec![
        Scope {
            a: 61925375349358592,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5021 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5020 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5071 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\{)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5022 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?=\\()"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5051 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\["),
      scope: vec![
        Scope {
            a: 47288521944400054,
            b: 20266198323167232,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5006 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5077 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("=>"),
      scope: vec![
        Scope {
            a: 52636628103462912,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5018 }),
    ]),
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 5072 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(";"),
      scope: vec![
        Scope {
            a: 47288689445306368,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5018 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[:,]"),
      scope: vec![
        Scope {
            a: 47288620725829632,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 5018 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\."),
      scope: vec![
        Scope {
            a: 47288788226932808,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }