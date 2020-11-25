
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
        ContextReference::Direct(ContextId { index: 5660 }),
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
        ContextReference::Direct(ContextId { index: 5631 }),
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
        ContextReference::Direct(ContextId { index: 5643 }),
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
        ContextReference::Direct(ContextId { index: 5625 }),
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
        ContextReference::Direct(ContextId { index: 5596 }),
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
        ContextReference::Direct(ContextId { index: 5671 }),
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
        ContextReference::Direct(ContextId { index: 5598 }),
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
        ContextReference::Direct(ContextId { index: 5646 }),
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
        ContextReference::Direct(ContextId { index: 5628 }),
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
        ContextReference::Direct(ContextId { index: 5683 }),
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
        ContextReference::Direct(ContextId { index: 5686 }),
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
        ContextReference::Direct(ContextId { index: 5668 }),
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
        ContextReference::Direct(ContextId { index: 5590 }),
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
        ContextReference::Direct(ContextId { index: 5619 }),
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
        ContextReference::Direct(ContextId { index: 5680 }),
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
        ContextReference::Direct(ContextId { index: 5584 }),
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
        ContextReference::Direct(ContextId { index: 5657 }),
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
        ContextReference::Direct(ContextId { index: 5617 }),
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
        ContextReference::Direct(ContextId { index: 5652 }),
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
        ContextReference::Direct(ContextId { index: 5612 }),
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
        ContextReference::Direct(ContextId { index: 5649 }),
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
        ContextReference::Direct(ContextId { index: 5640 }),
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
        ContextReference::Direct(ContextId { index: 5614 }),
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
        ContextReference::Direct(ContextId { index: 5622 }),
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
        ContextReference::Direct(ContextId { index: 5604 }),
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
        ContextReference::Direct(ContextId { index: 5609 }),
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
        ContextReference::Direct(ContextId { index: 5606 }),
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
        ContextReference::Direct(ContextId { index: 5695 }),
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
        ContextReference::Direct(ContextId { index: 5634 }),
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
        ContextReference::Direct(ContextId { index: 5587 }),
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
        ContextReference::Direct(ContextId { index: 5593 }),
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
        ContextReference::Direct(ContextId { index: 5674 }),
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
        ContextReference::Direct(ContextId { index: 5689 }),
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
        ContextReference::Direct(ContextId { index: 5601 }),
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
        ContextReference::Direct(ContextId { index: 5677 }),
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
        ContextReference::Direct(ContextId { index: 5662 }),
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
        ContextReference::Direct(ContextId { index: 5637 }),
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
        ContextReference::Direct(ContextId { index: 5655 }),
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
        ContextReference::Direct(ContextId { index: 5665 }),
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
        ContextReference::Direct(ContextId { index: 5692 }),
    ]),
      with_prototype: None
    }),
]
} }