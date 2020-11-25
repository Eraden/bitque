
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use jirs_syntect::Pattern::*;
use std::cell::*;

#[inline(always)]
pub fn load() -> Context {
        Context {
  meta_scope: vec![
    Scope {
        a: 844970390978560,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844970390978560,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\#\\[\\s*(TODO|todo)?)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629469118591,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314812305535,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8395 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(##\\s*(TODO|todo)?).+(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711032875382,
            b: 35747322042253312,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629469773951,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314812305535,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(##\\s*)(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711032875382,
            b: 206321703389757440,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629469774557,
            b: 35747322042253312,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#\\s*(TODO|todo)?).*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711032873143,
            b: 35747322042253312,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038847,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 50103314812305535,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{\\."),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8396 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("discard \\\"\\\"\\\""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8402 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(\\d[_\\d]*((\\.[_\\d]+([eE][\\+\\-]?\\d[_\\d]*)?)|([eE][\\+\\-]?\\d[_\\d]*)))(\'?([fF](32|64|128))|[fFdD])?"),
      scope: vec![
        Scope {
            a: 59955089176592602,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(0[xX][0-9A-Fa-f][_0-9A-Fa-f]*)(\'?(([iIuUfF](8|16|32|64))|[uUfFdD]))?"),
      scope: vec![
        Scope {
            a: 59955089176461528,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(0[ocC][0-7][_0-7]*)(\'?(([iIuUfF](8|16|32|64))|[uUfFdD]))?"),
      scope: vec![
        Scope {
            a: 59955089176461666,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(0(b|B)[01][_01]*)(\'?(([iIuUfF](8|16|32|64))|[uUfFdD]))?"),
      scope: vec![
        Scope {
            a: 59955089176461741,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(\\d[_\\d]*)(\'?(([iIuUfF](8|16|32|64))|[uUfFdD]))?"),
      scope: vec![
        Scope {
            a: 59955089176461530,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(true|false|inf|nil)(?![\\w\\x{80}-\\x{10FFFF}])"),
      scope: vec![
        Scope {
            a: 59955110645530624,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:^|\\s+|=)(when|if)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636828336255,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8403 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:^|\\s+|=)(case)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636828336255,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8404 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?:^|\\s+)(of|else|elif)\\b"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636828401791,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8405 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(await|block|break|continue|do|end|except|finally|raise|return|try|while|yield)(?![\\w\\x{80}-\\x{10FFFF}])"),
      scope: vec![
        Scope {
            a: 52636636697001984,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?<![\\w\\x{80}-\\x{10FFFF}])(and|in|is|isnot|not|notin|or|xor)(?![\\w\\x{80}-\\x{10FFFF}]))"),
      scope: vec![
        Scope {
            a: 52636628118732927,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?<![\\w\\x{80}-\\x{10FFFF}])(addr|as|atomic|bind|cast|const|converter|defer|discard|distinct|div|enum|export|include|let|mixin|object|of|ptr|ref|shl|shr|static|type|var)(?![\\w\\x{80}-\\x{10FFFF}]))"),
      scope: vec![
        Scope {
            a: 52636787020857344,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\b()\\b|(=|\\+|-|\\*|/|<|>|@|\\$|~|&|%|!|\\?|\\^|\\.|:|\\\\)+)"),
      scope: vec![
        Scope {
            a: 52636628107067392,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(for)\\s+"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636697001984,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8406 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((import)\\s+[\\.|\\w|\\/]+,?)"),
      scope: vec![],
      captures: Some(vec![(2, vec![
        Scope {
            a: 52636636697001984,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(from)\\s+[\\/\\w]+\\s+(?=import)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636697001984,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(var|let)\\s*\\("),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787020857344,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8407 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?<![\\w\\x{80}-\\x{10FFFF}])(generic|interface|lambda|out|shared|with|without)(?![\\w\\x{80}-\\x{10FFFF}]))"),
      scope: vec![
        Scope {
            a: 52636585157394432,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(new|GC_ref|GC_unref|assert|echo|defined|declared|newException|countup|countdown|len|high|low)(?![\\w\\x{80}-\\x{10FFFF}])"),
      scope: vec![
        Scope {
            a: 52636787014107313,
            b: 35747322042253312,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(((uint|int|float)(8|16|32|64)?)|clong|culong|cchar|cschar|cshort|cint|csize|clonglong|cfloat|cdouble|clongdouble|cuchar|cushort|cuint|culonglong|cstringArray|bool|string|auto|cstring|char|byte|tobject|typedesc|stmt|expr|any|untyped|typed)(?![\\w\\x{80}-\\x{10FFFF}])"),
      scope: vec![
        Scope {
            a: 48414576621322367,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(range|array|seq|tuple|natural|set|ref|ptr|pointer)(?![\\w\\x{80}-\\x{10FFFF}])"),
      scope: vec![
        Scope {
            a: 48414576481468543,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(proc|iterator|method|template|macro)(?![\\w\\x{80}-\\x{10FFFF}])"),
      scope: vec![
        Scope {
            a: 48414576474128511,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(openarray|varargs|void)(?![\\w\\x{80}-\\x{10FFFF}])"),
      scope: vec![
        Scope {
            a: 48414576481468543,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])([A-Z][A-Z0-9_]+)(?![\\w\\x{80}-\\x{10FFFF}])"),
      scope: vec![
        Scope {
            a: 61925409712701440,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])([A-Z]\\w+)(?![\\w\\x{80}-\\x{10FFFF}])"),
      scope: vec![
        Scope {
            a: 61925375352963072,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![\\w\\x{80}-\\x{10FFFF}])(\\w+)(?![\\w\\x{80}-\\x{10FFFF}])(?=\\()"),
      scope: vec![
        Scope {
            a: 61925255157907583,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("r?\\\"\\\"\\\""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8408 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("r\\\""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8397 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8398 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\'"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8399 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([\\w\\x{80}-\\x{10FFFF}\\`]+)\\s*(?=\\(|\\[.+?\\]\\s*\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255157907583,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8400 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([\\w\\x{80}-\\x{10FFFF}]+)(?=\\s+[\\w](?![\\w\\x{80}-\\x{10FFFF}]+((?!\\n)\\s)+))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255157907583,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([\\w\\x{80}-\\x{10FFFF}]+)(?=\\s+[\\\"\\\'\\`])"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255157907583,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(tmpl(i)?)(?=( (nim|html|xml|js|css|glsl|md))?\\\"\\\"\\\")"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576474128511,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628107067392,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }