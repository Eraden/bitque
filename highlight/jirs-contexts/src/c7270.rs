
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
        a: 844884491632640,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 844884491632640,
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
      regex: Regex::new("^[Cc]"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7265 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\!"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7265 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\*"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7265 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^     (\\d|\\*)"),
      scope: vec![
        Scope {
            a: 51519571543591109,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\\'"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7274 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\\")"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7267 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (use)  \\s+  (\\w+)  ( (,)  \\s*  (only)  \\s*  (:) )?  )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 316097296641359872,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52640644040294400,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576512534725,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 295267744564314321,
            b: 631911322715422720,
        },
    ]),(5, vec![
        Scope {
            a: 52639321190367232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:(implicit)  \\s+  (none|.*)  )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 626282720976502784,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52645377094254592,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46453026390867968,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:^  \\s*  ([!]\\$)  .*  (?m:$))"),
      scope: vec![
        Scope {
            a: 61925461251262661,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?i:(program))\\b\\s+(\\p{Alpha}\\w*)\\s*(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576609921221,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130630617289,
            b: 631911322715422720,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7273 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (module|type)  \\s+  (\\w+)  \\s*  (?m:$))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52639097852067840,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632452293,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (end)  \\s+  (module|type)  \\s+  (\\w+)?  \\s*  (?m:$))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52639097852067840,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52639097852067840,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 48416973201408000,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  ([a-zA-Z\\(\\)]*)  (?<!end)  \\s*  (function|subroutine)\\b  \\s+  (\\p{Alpha}\\w*)  \\s*  (\\(|&) )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576609656832,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576474130629,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630617285,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7275 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  ([a-zA-Z\\(\\)]*)  (?<!end)  \\s*  (subroutine)\\b  \\s+  (\\p{Alpha}\\w*)  \\s*  (?m:$))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576609656832,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576474130629,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 59392130630617285,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (if|else\\s*if|elif|endif)  \\b  )"),
      scope: vec![
        Scope {
            a: 52636636835807232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (do)  \\b  (\\s+\\d+\\s+)?  \\s*  )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46444672679477248,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52636636835807232,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258876985737216,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:(assign|to|if|then|else|elseif|elif|select\\s*case|case|end\\s*select|end\\s*if|continue|stop|pause|do|end\\s*do|while|cycle))\\b"),
      scope: vec![
        Scope {
            a: 52636636835807232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:(entry|return|contains|include))\\b"),
      scope: vec![
        Scope {
            a: 52636636836137157,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("=>"),
      scope: vec![
        Scope {
            a: 52636628245872640,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(==|<=|>=)"),
      scope: vec![
        Scope {
            a: 52636628246268101,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\s=\\s"),
      scope: vec![
        Scope {
            a: 52636628245872640,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:(?<!\\w)  [-+]?  (\\d+ | \\d*\\.\\d+ | \\d+\\.\\d*)  ([ed][-+]?\\d+)  \\b)"),
      scope: vec![
        Scope {
            a: 59955089232496837,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:=  |  -  |  \\+  |  /  |  \\*  |  //  |  ::)"),
      scope: vec![
        Scope {
            a: 52636628245872640,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\.  (and|or|eq|lt|le|gt|ge|ne|not|eqv|neqv)  \\.)"),
      scope: vec![
        Scope {
            a: 52636628114802885,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:(integer|real|complex|logical|character|double\\s+precision))\\b"),
      scope: vec![
        Scope {
            a: 48414576609656832,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:(operator|assignment))\\b"),
      scope: vec![
        Scope {
            a: 48414576609656832,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (intent)  \\s* \\(  \\s*  (in|out|inout)  \\s*  \\)  )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46453043570737152,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52645493058371584,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955110784335872,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (dimension)  \\s* \\(  [^)]*  \\)  )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46453047865704448,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52645497353338880,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (dimension)  \\b  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52645497353338880,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:(\\w+)  \\( (\\*) \\))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 49258876985737216,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258876985737216,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (\\d+)?  \\s*  (continue)  \\s*  (?m:$))"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46453052160671744,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 49258876985737216,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52637822246780928,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(    \\d|   \\d\\d|  \\d\\d\\d)"),
      scope: vec![
        Scope {
            a: 49258876985737216,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (go\\s*to)  \\s+  (\\d+))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52637633268219904,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49258876985737216,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:(allocatable|equivalence|parameter|external|intrinsic|save))\\b"),
      scope: vec![
        Scope {
            a: 48414439170703360,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (close|open)  \\s*  \\(  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (print|write|rewrite|read)  \\s*  \\(  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (print|write|rewrite|read)  \\s*  \\(  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (pause|wait|rewind|flush)  \\s*  \\(  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (\\d+)  \\s+  (format)  \\(  )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 350718718776770560,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 59955089309499392,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52641172321271808,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7268 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (allocate|deallocate)  \\b  \\s*  \\(  \\s*  (\\w+)  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 49259087439134720,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  \\#  \\s*  (define|if|ifdef|ifndef|else|endif)  \\b  .*  (?m:$))"),
      scope: vec![
        Scope {
            a: 52636636835807232,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  \\#  \\s*  (include|import)  \\b  )"),
      scope: vec![
        Scope {
            a: 52636636717449670,
            b: 631911322715422720,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7272 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  \\#  \\s*  (error|warning)  \\b  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636717449485,
            b: 631911322715422720,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7271 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (common)  \\s*  (/\\w+/)  \\s*  )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 6756297236348928,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52635923871236096,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576610314437,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7266 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (common)  \\s+  )"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 6756297236348928,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 52635923871236096,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7266 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (data)  \\s+  (\\w+)  \\s*  (/))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52637096397307904,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414971746648064,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (block\\s*data)  \\s+  (\\w+)  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52645510238240768,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48423385587580928,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 7264 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:\\.true\\.|\\.false\\.)"),
      scope: vec![
        Scope {
            a: 59955110784335872,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b[0-9.]*\\b"),
      scope: vec![
        Scope {
            a: 59955089309499392,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:present)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:abs|aimag|aint|anint|cmplx|conjg|dble|dim|dprod|int|max|min|mod|nint|real|sign)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:ceiling|floor|modulo)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:acos|asin|atan|atan2|cos|cosh|exp|log|log10|sin|sinh|sqrt|tan|tanh)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:achar|adjustl|adjustr|char|iachar|ichar|index|len_trim|lge|lgt|lle|llt|repeat|scan|trim|verify)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:len)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:kind|selected_int_kind|selected_real_kind)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:logical)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:digits|epsilon|huge|maxexponent|minexponent|precision|radix|range|tiny)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:bit_size)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:btest|iand|ibclr|ibits|ibset|ieor|ior|ishft|ishftc|not)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:transfer)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:exponent|fraction|nearest|rrspacing|scale|set_exponent|spacing)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:dot_product|matmul)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:all|any|count|maxval|minval|product|sum)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:allocated|lbound|shape|size|ubound)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:merge)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:reshape)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:cshift|eoshift|transpose)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:maxloc|minloc)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (?i:associated)  \\b  (?=\\s*\\()  )"),
      scope: vec![
        Scope {
            a: 61925255232684032,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:\\b  (call)  \\s+  (?i:date_and_time|system_clock|count_max|mvbits|random_number|random_seed)  \\b  (?=\\s*\\()  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636877353975808,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:\\b  (call)  \\s*  (\\w+)  \\s*  (?=\\(|(?m:$))  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636877353975808,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46453065045573632,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x:(\\p{Alpha}\\w+)  \\s*  (?=\\()  )"),
      scope: vec![
        Scope {
            a: 46444883132874752,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (end)  ( (?m:$) | \\s*(function|subroutine|module|type) (\\s+\\p{Alpha}\\w*)? )  )"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52645518828175360,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414576609656832,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?xi:^  \\s*  (end)  \\s*  (?m:$))"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636835807232,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }