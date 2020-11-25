
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
      regex: Regex::new("((\\\\)end)(\\{)(minted)(\\})"),
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
      regex: Regex::new("(\\{)(c)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2835 }),
        ContextReference::Direct(ContextId { index: 777 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2846 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(cpp|c\\+\\+)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2857 }),
        ContextReference::Direct(ContextId { index: 610 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2868 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(diff)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2877 }),
        ContextReference::Direct(ContextId { index: 1373 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2878 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(go|golang)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2879 }),
        ContextReference::Direct(ContextId { index: 1879 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2880 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(haskell|hs)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2881 }),
        ContextReference::Direct(ContextId { index: 2173 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2836 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(html)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2837 }),
        ContextReference::Direct(ContextId { index: 2107 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2838 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(java)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2839 }),
        ContextReference::Direct(ContextId { index: 2353 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2840 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(javascript|js)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2841 }),
        ContextReference::Direct(ContextId { index: 7848 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2842 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(json)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2843 }),
        ContextReference::Direct(ContextId { index: 2200 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2844 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(tex|latex)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2845 }),
        ContextReference::Direct(ContextId { index: 2968 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2847 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(lisp)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2848 }),
        ContextReference::Direct(ContextId { index: 3031 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2849 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(lua)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2850 }),
        ContextReference::Direct(ContextId { index: 3106 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2851 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(obj(?:ective\\-|)c)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2852 }),
        ContextReference::Direct(ContextId { index: 3925 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2853 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(obj(?:ective\\-|)c\\+\\+)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2854 }),
        ContextReference::Direct(ContextId { index: 3754 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2855 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(perl)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2856 }),
        ContextReference::Direct(ContextId { index: 4323 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2858 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(php)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2859 }),
        ContextReference::Direct(ContextId { index: 4135 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2860 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(python|py)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2861 }),
        ContextReference::Direct(ContextId { index: 4613 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2862 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(r)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2863 }),
        ContextReference::Direct(ContextId { index: 4674 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2864 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(ruby)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2865 }),
        ContextReference::Direct(ContextId { index: 4920 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2866 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(sh|shell|bash )(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2867 }),
        ContextReference::Direct(ContextId { index: 5464 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2869 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(sql|mysql|ddl|dml)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2870 }),
        ContextReference::Direct(ContextId { index: 5102 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2871 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(xml)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2872 }),
        ContextReference::Direct(ContextId { index: 5819 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2873 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(yaml)(\\})"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(2, vec![
        Scope {
            a: 49258876850208805,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582708,
            b: 48132379931312128,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2874 }),
        ContextReference::Direct(ContextId { index: 5881 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2875 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2876 }),
    ]),
      with_prototype: None
    }),
]
} }