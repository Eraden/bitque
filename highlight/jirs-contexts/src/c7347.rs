
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
]
} }