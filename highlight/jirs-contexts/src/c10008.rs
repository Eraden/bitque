
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
        a: 845077765160960,
        b: 0,
    },
],
  meta_content_scope: vec![
    Scope {
        a: 845077765160960,
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
      regex: Regex::new("^\\s*(module|function|primitive)\\s+\\b([a-zA-Z_][a-zA-Z0-9_]*)\\b"),
      scope: vec![
        Scope {
            a: 46444277445361816,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 48414576472489984,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450251,
            b: 42784196460019712,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?!else)(?!begin)([a-zA-Z_][a-zA-Z0-9_]*)\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\s*(\\()"),
      scope: vec![
        Scope {
            a: 46444204390875136,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450251,
            b: 42784196460019712,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450433,
            b: 42784196460019712,
        },
    ]),(3, vec![
        Scope {
            a: 52636628154451214,
            b: 42784196460019712,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(?!else)(?!begin)([a-zA-Z_][a-zA-Z0-9_]*)\\s+([#])(\\()([ ._+`,a-zA-Z0-9]+)(\\))\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\s*(\\()"),
      scope: vec![
        Scope {
            a: 46444204548882584,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450251,
            b: 42784196460019712,
        },
    ]),(2, vec![
        Scope {
            a: 52636787022495744,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 52636628154451214,
            b: 42784196460019712,
        },
    ]),(4, vec![
        Scope {
            a: 52636787022495744,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 52636628154451214,
            b: 42784196460019712,
        },
    ]),(6, vec![
        Scope {
            a: 59392130632450433,
            b: 42784196460019712,
        },
    ]),(7, vec![
        Scope {
            a: 52636628154451214,
            b: 42784196460019712,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*\\b(?!else)(?!begin)([a-zA-Z_][a-zA-Z0-9_]*)\\b\\s+([#])"),
      scope: vec![
        Scope {
            a: 46444204548882584,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130632450251,
            b: 42784196460019712,
        },
    ]),(2, vec![
        Scope {
            a: 52636787022495744,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(defparam)\\s+([a-zA-Z_][a-zA-Z0-9_]*)(.[a-zA-Z_][a-zA-Z0-9_]*)\\s*(=)"),
      scope: vec![
        Scope {
            a: 46444204548948120,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636787022495744,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450433,
            b: 42784196460019712,
        },
    ]),(3, vec![
        Scope {
            a: 46446648233820312,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 52636787022495744,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(automatic|cell|config|deassign|defparam|design|disable|edge|endconfig|endgenerate|endspecify|endtable|endtask|event|generate|genvar|ifnone|incdir|include|instance|liblist|library|localparam|macromodule|negedge|noshowcancelled|posedge|pulsestyle_onevent|pulsestyle_ondetect|real|realtime|scalared|showcancelled|specify|specparam|table|task|time|use|vectored)\\b"),
      scope: vec![
        Scope {
            a: 52636787022495744,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#[0-9]+)"),
      scope: vec![
        Scope {
            a: 52646837245968384,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(initial|always|wait|force|release|assign)\\b"),
      scope: vec![
        Scope {
            a: 52636636698640384,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(begin|end|fork|join)\\b"),
      scope: vec![
        Scope {
            a: 52636787022495744,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(forever|repeat|while|for|if|else|case|casex|casez|default|endcase)\\b"),
      scope: vec![
        Scope {
            a: 52636636698640384,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(`include)\\s+([\"<].*[\">])"),
      scope: vec![
        Scope {
            a: 46445321082372096,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466383880192,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59392130632450502,
            b: 42784196460019712,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*(`ifdef|`ifndef|`undef|`define)\\s+([a-zA-Z_][a-zA-Z0-9_]*)\\b"),
      scope: vec![
        Scope {
            a: 46444466542084248,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 46444466383880192,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 59955136437420184,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("`(celldefine|default_nettype|define|else|elsif|endcelldefine|endif|ifdef|ifndef|include|line|nounconnected_drive|resetall|timescale|unconnected_drive|undef)\\b"),
      scope: vec![
        Scope {
            a: 46444466383880192,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[.][_a-zA-Z0-9]+"),
      scope: vec![
        Scope {
            a: 46446648233820312,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("`\\b([a-zA-Z_][a-zA-Z0-9_]*)\\b"),
      scope: vec![
        Scope {
            a: 59955136437420184,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10007 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(endmodule|endfunction|endprimitive)\\b"),
      scope: vec![
        Scope {
            a: 48414576472489984,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s*\\b([a-zA-Z_][a-zA-Z0-9_]*)\\b\\s*(:)\\s*"),
      scope: vec![
        Scope {
            a: 46445063384334336,
            b: 0,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 59392130787246232,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 52636628135903384,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10006 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(==|===|!=|!==|<=|>=|<|>)"),
      scope: vec![
        Scope {
            a: 52636628119257240,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\-|\\+|\\*|\\/|%)"),
      scope: vec![
        Scope {
            a: 52636628119191704,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(!|&&|\\|\\|)"),
      scope: vec![
        Scope {
            a: 52636628114800792,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(&|\\||\\^|~|<<|>>|\\?|:)"),
      scope: vec![
        Scope {
            a: 52636628135903384,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("({|})"),
      scope: vec![
        Scope {
            a: 52636628154450892,
            b: 42784196460019712,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\(|\\))"),
      scope: vec![
        Scope {
            a: 52636628154451214,
            b: 42784196460019712,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\[|\\])"),
      scope: vec![
        Scope {
            a: 52636628154451215,
            b: 42784196460019712,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("([;,])"),
      scope: vec![
        Scope {
            a: 52642473559195648,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(#|@|=)"),
      scope: vec![
        Scope {
            a: 52636787022495744,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(output|input|inout|and|nand|nor|or|xor|xnor|buf|not|bufif[01]|notif[01]|r?[npc]mos|tran|r?tranif[01]|pullup|pulldown)\\b"),
      scope: vec![
        Scope {
            a: 61925375354601472,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("((\\b\\d+)?\'s?([bB]\\s*(([0-1_xXzZ?]+)|(`[A-Z]+[_0-9a-zA-Z]*))|[oO]\\s*(([0-7_xXzZ?]+)|(`[A-Z]+[_0-9a-zA-Z]*))|[dD]\\s*(([0-9_xXzZ?]+)|(`[A-Z]+[_0-9a-zA-Z]*))|[hH]\\s*(([0-9a-fA-F_xXzZ?]+)|(`[A-Z]+[_0-9a-zA-Z]*)))((e|E)(\\+|-)?[0-9]+)?\\b)|(\\b\\d+\\b)"),
      scope: vec![
        Scope {
            a: 59955089172332544,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 10011 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\$\\b([a-zA-Z_][a-zA-Z0-9_]*)\\b"),
      scope: vec![
        Scope {
            a: 61925255095517184,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }