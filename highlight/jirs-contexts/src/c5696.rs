
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
        a: 21392428947341387,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 21392428947341387,
        b: 0,
    },
],
  meta_include_prototype: true,
  clear_scopes: None,
  prototype: Some(
    ContextId {
        index: 5698,
    },
),
  uses_backrefs: false,
  patterns: vec![
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("set(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255101743178,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5658 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("help(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180058698,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5629 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("mapfile(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180124234,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5641 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("getopts(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180189770,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5623 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("cd(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180255306,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5594 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("suspend(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180320842,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5669 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new(":(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255107969098,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5597 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("printf(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180386378,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5644 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("hash(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255111180362,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5626 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("ulimit(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180451914,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5681 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("umask(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180517450,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5684 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("source(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255085752394,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5666 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("builtin(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255090602058,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5588 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("exit(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255168065610,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5618 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("type(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255098990666,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5678 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("bg(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180582986,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5582 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("return(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 52636636701196639,
            b: 20829148276588544,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5656 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("exec(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255131037770,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5615 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("read(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180648522,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5650 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("enable(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180714058,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5610 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("pwd(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180779594,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5647 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("logout(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180845130,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5638 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("eval(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180910666,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5613 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("fg(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255180976202,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5620 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("disown(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181041738,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5602 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("echo(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181107274,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5607 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\.(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255087652938,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5605 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("wait(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255168327754,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5693 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("jobs(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181172810,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5632 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("bind(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181238346,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5585 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("caller(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255167869002,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5591 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("times(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255103643722,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5672 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("unalias(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181303882,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5687 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("command(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255106265162,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5599 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("trap(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181369418,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5675 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("shift(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181434954,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5661 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("kill(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181500490,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5635 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("readarray(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181566026,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5653 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("shopt(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181631562,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5663 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("unset(?=\\s|;|(?m:$)|>|<|\\|)"),
      scope: vec![
        Scope {
            a: 46444882990596096,
            b: 0,
        },
        Scope {
            a: 61925255181697098,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Set(vec![
        ContextReference::Direct(ContextId { index: 5391 }),
        ContextReference::Direct(ContextId { index: 5690 }),
    ]),
      with_prototype: None
    }),
]
} }