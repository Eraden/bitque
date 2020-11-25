
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
  meta_include_prototype: false,
  clear_scopes: None,
  prototype: None,
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\\\)end)(\\{)(lstlisting)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255096762405,
            b: 0,
        },
        Scope {
            a: 52636636701196459,
            b: 10414574138294272,
        },
    ]),(2, vec![
        Scope {
            a: 47288629362032677,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(4, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2960 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:c))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2921 }),
        ContextReference::Direct(ContextId { index: 777 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2922 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:cpp|c\\+\\+))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2923 }),
        ContextReference::Direct(ContextId { index: 610 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2924 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:haskell|hs))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2925 }),
        ContextReference::Direct(ContextId { index: 2173 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2926 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:java))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2890 }),
        ContextReference::Direct(ContextId { index: 2353 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2891 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:html))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2892 }),
        ContextReference::Direct(ContextId { index: 2107 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2893 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:tex|latex))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2894 }),
        ContextReference::Direct(ContextId { index: 2968 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2895 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:lisp))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2896 }),
        ContextReference::Direct(ContextId { index: 3031 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2897 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:lua))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2898 }),
        ContextReference::Direct(ContextId { index: 3106 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2899 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:perl))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2901 }),
        ContextReference::Direct(ContextId { index: 4323 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2902 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:php))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2903 }),
        ContextReference::Direct(ContextId { index: 4135 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2904 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:python|py))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2905 }),
        ContextReference::Direct(ContextId { index: 4613 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2906 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:r))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2907 }),
        ContextReference::Direct(ContextId { index: 4674 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2908 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:ruby))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2909 }),
        ContextReference::Direct(ContextId { index: 4920 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2910 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:sh|shell|bash ))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2912 }),
        ContextReference::Direct(ContextId { index: 5464 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2913 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:sql|mysql|ddl|dml))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2914 }),
        ContextReference::Direct(ContextId { index: 5102 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2915 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:xml))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2916 }),
        ContextReference::Direct(ContextId { index: 5819 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2917 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(".*(%\\s*(?i:yaml))(?m:$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 51510711050698789,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2918 }),
        ContextReference::Direct(ContextId { index: 5881 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2919 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2920 }),
    ]),
      with_prototype: None
    }),
]
} }