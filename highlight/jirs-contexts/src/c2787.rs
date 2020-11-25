
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
        index: 2981,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Include(ContextReference::Direct(ContextId { index: 2960 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(c)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2788 }),
        ContextReference::Direct(ContextId { index: 777 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2799 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(cpp|c\\+\\+)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2810 }),
        ContextReference::Direct(ContextId { index: 610 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2821 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(diff)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2829 }),
        ContextReference::Direct(ContextId { index: 1373 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2830 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(go|golang)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2831 }),
        ContextReference::Direct(ContextId { index: 1879 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2832 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(haskell|hs)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2833 }),
        ContextReference::Direct(ContextId { index: 2173 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2789 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(html)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2790 }),
        ContextReference::Direct(ContextId { index: 2107 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2791 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(java)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2792 }),
        ContextReference::Direct(ContextId { index: 2353 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2793 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(javascript|js)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2794 }),
        ContextReference::Direct(ContextId { index: 7848 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2795 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(json)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2796 }),
        ContextReference::Direct(ContextId { index: 2200 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2797 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(tex|latex)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2798 }),
        ContextReference::Direct(ContextId { index: 2968 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2800 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(lisp)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2801 }),
        ContextReference::Direct(ContextId { index: 3031 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2802 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(lua)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2803 }),
        ContextReference::Direct(ContextId { index: 3106 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2804 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(obj(?:ective\\-|)c)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2805 }),
        ContextReference::Direct(ContextId { index: 3925 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2806 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(obj(?:ective\\-|)c\\+\\+)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2807 }),
        ContextReference::Direct(ContextId { index: 3754 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2808 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(perl)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2809 }),
        ContextReference::Direct(ContextId { index: 4323 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2811 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(php)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2812 }),
        ContextReference::Direct(ContextId { index: 4135 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2813 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(python|py)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2814 }),
        ContextReference::Direct(ContextId { index: 4613 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2815 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(r)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2816 }),
        ContextReference::Direct(ContextId { index: 4674 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2817 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(ruby)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2818 }),
        ContextReference::Direct(ContextId { index: 4920 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2819 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(sh|shell|bash )(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2820 }),
        ContextReference::Direct(ContextId { index: 5464 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2822 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(sql|mysql|ddl|dml)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2823 }),
        ContextReference::Direct(ContextId { index: 5102 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2824 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(xml)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2825 }),
        ContextReference::Direct(ContextId { index: 5819 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2826 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\{)(yaml)(\\})((\\{)|(\\W))"),
      scope: vec![
        Scope {
            a: 46446119994065778,
            b: 10414574138294272,
        },
    ],
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
    ]),(5, vec![
        Scope {
            a: 47288629318582708,
            b: 51228604675129344,
        },
    ]),(6, vec![
        Scope {
            a: 47288629368913957,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 2827 }),
        ContextReference::Direct(ContextId { index: 5881 }),
    ]),
      with_prototype: Some(
        ContextReference::Direct(ContextId { index: 2828 }),
    )
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(""),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }