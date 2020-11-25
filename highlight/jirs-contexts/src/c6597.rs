
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
        a: 844820067123200,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844820067123200,
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
      regex: Regex::new("(\\([^()]*?\\))\\s*([=-]>)"),
      scope: vec![
        Scope {
            a: 46446605279232092,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258876850208860,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576474128476,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(new)\\s+(\\w+(?:\\.\\w*)*)"),
      scope: vec![
        Scope {
            a: 46444243060851052,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628113752156,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925366760734720,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\'\'\'"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 25895697857380352,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6583 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\"\"\""),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 25895697857380352,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6584 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("`"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323956406,
            b: 25895697857380352,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6585 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<!#)###(?!#)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 47288629323038812,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6586 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#)(?!\\{).*(?m:$)\\n?"),
      scope: vec![
        Scope {
            a: 51510711032873052,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629323038812,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("/{3}"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6587 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("/(?![\\s=/*+{}?]).*?[^\\\\]/[igmy]{0,4}(?![a-zA-Z0-9])"),
      scope: vec![
        Scope {
            a: 55450759396589568,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n  \\b(?<![\\.\\$])(\n    break|by|catch|continue|else|finally|for|in|of|if|return|switch|\n    then|throw|try|unless|when|while|until|loop|do|(?<=for)\\s+own\n  )(?!\\s*:)\\b"),
      scope: vec![
        Scope {
            a: 52636636694708224,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n  and=|or=|!|%|&|\\^|\\*|\\/|(\\-)?\\-(?!>)|\\+\\+|\\+|~|==|=(?!>)|!=|<=|>=|<<=|>>=|\n  >>>=|<>|<|>|!|&&|\\.\\.(\\.)?|\\?|\\||\\|\\||\\:|\\*=|(?<!\\()/=|%=|\\+=|\\-=|&=|\n  \\^=|\\b(?<![\\.\\$])(instanceof|new|delete|typeof|and|or|is|isnt|not|super)\\b"),
      scope: vec![
        Scope {
            a: 52636628104773632,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([a-zA-Z\\$_](\\w|\\$|\\.)*\\s*(?!\\::)((:)|(=[^=]))(?!(\\s*\\(.*\\))?\\s*((=|-)>)))"),
      scope: vec![
        Scope {
            a: 49258932679213056,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258932679213056,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288620737429504,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636628104773632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<=\\s|^)([\\[\\{])(?=.*?[\\]\\}]\\s+[:=])"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 52636628104773632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6588 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\n  (\\s*)\n  (?=[a-zA-Z\\$_])\n  (\n    [a-zA-Z\\$_](\\w|\\$|:|\\.)*\\s*\n    (?=[:=](\\s*\\(.*\\))?\\s*([=-]>))\n  )"),
      scope: vec![
        Scope {
            a: 46444131372498944,
            b: 0,
        },
    ],
      captures: Some(vec![(2, vec![
        Scope {
            a: 59392130630615132,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630615132,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(describe|it|app\\.(get|post|put|all|del|delete))"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 6589 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[=-]>"),
      scope: vec![
        Scope {
            a: 48414576474128476,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?<!\\.)(true|on|yes)(?!\\s*[:=])\\b"),
      scope: vec![
        Scope {
            a: 59955110657196855,
            b: 25895697857380352,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?<!\\.)(false|off|no)(?!\\s*[:=])\\b"),
      scope: vec![
        Scope {
            a: 59955110657196856,
            b: 25895697857380352,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?<!\\.)null(?!\\s*[:=])\\b"),
      scope: vec![
        Scope {
            a: 59955110657261660,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?<!\\.)(this|extends)(?!\\s*[:=])\\b"),
      scope: vec![
        Scope {
            a: 49259061528231936,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(class\\b)\\s+(@?[a-zA-Z\\$_][\\w\\.]*)?(?:\\s+(extends)\\s+(@?[a-zA-Z\\$\\._][\\w\\.]*))?"),
      scope: vec![
        Scope {
            a: 46444243041648640,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576475832412,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450251,
            b: 25895697857380352,
        },
    ]),(3, vec![
        Scope {
            a: 52636636752642140,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 59392186470432860,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(debugger|\\\\)\\b"),
      scope: vec![
        Scope {
            a: 52636787018563584,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\\b(\n  Array|ArrayBuffer|Blob|Boolean|Date|document|event|Function|\n  Int(8|16|32|64)Array|Math|Map|Number|\n  Object|Proxy|RegExp|Set|String|WeakMap|\n  window|Uint(8|16|32|64)Array|XMLHttpRequest\n)\\b"),
      scope: vec![
        Scope {
            a: 61925366760734720,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((?<=console\\.)(debug|warn|info|log|error|time|timeEnd|assert))\\b"),
      scope: vec![
        Scope {
            a: 61925255139360860,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)\\b(\n  decodeURI(Component)?|encodeURI(Component)?|eval|parse(Float|Int)|require\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255091585024,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)((?<=\\.)(\n  apply|call|concat|every|filter|forEach|from|hasOwnProperty|indexOf|\n  isPrototypeOf|join|lastIndexOf|map|of|pop|propertyIsEnumerable|push|\n  reduce(Right)?|reverse|shift|slice|some|sort|splice|to(Locale)?String|\n  unshift|valueOf\n))\\b"),
      scope: vec![
        Scope {
            a: 61925255096434869,
            b: 25895697857380352,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)((?<=Array\\.)(\n  isArray\n))\\b"),
      scope: vec![
        Scope {
            a: 61925255117209781,
            b: 25895697857380352,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)((?<=Object\\.)(\n  create|definePropert(ies|y)|freeze|getOwnProperty(Descriptors?|Names)|\n  getProperty(Descriptor|Names)|getPrototypeOf|is(Extensible|Frozen|Sealed)?|\n  isnt|keys|preventExtensions|seal\n))\\b"),
      scope: vec![
        Scope {
            a: 61925255117210430,
            b: 25895697857380352,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)((?<=Math\\.)(\n  abs|acos|acosh|asin|asinh|atan|atan2|atanh|ceil|cos|cosh|exp|expm1|floor|\n  hypot|log|log10|log1p|log2|max|min|pow|random|round|sign|sin|sinh|sqrt|\n  tan|tanh|trunc\n))\\b"),
      scope: vec![
        Scope {
            a: 61925255117210468,
            b: 25895697857380352,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)((?<=Number\\.)(\n  is(Finite|Integer|NaN)|toInteger\n))\\b"),
      scope: vec![
        Scope {
            a: 61925255117211716,
            b: 25895697857380352,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(Infinity|NaN|undefined)\\b"),
      scope: vec![
        Scope {
            a: 59955110643236864,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\;"),
      scope: vec![
        Scope {
            a: 47288689454284892,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(",[ |\\t]*"),
      scope: vec![
        Scope {
            a: 46450024115995174,
            b: 25895697857380352,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\."),
      scope: vec![
        Scope {
            a: 46450024072480842,
            b: 25895697857380352,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\{|\\}"),
      scope: vec![
        Scope {
            a: 46445243826700380,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\(|\\)"),
      scope: vec![
        Scope {
            a: 46445243847802972,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\[|\\]\\s*"),
      scope: vec![
        Scope {
            a: 46445243847868508,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6595 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6599 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6593 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 6598 })),
]
} }