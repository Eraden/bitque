
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
      regex: Regex::new("(?x)                         # pop out of this context when one of the following conditions are met:\n^(?:\n    \\s*(?m:$)                     # the line is blank (or only contains whitespace)\n|   (?=\n        (?:[ ]{,3}>(?:.|(?m:$)))      # a block quote begins the line\n    |   [ ]{,3}[*+-][ ]      # an unordered list item begins the line\n    |   [ ]{,3}1[.][ ]       # an ordered list item with number \"1\" begins the line\n    |   \\#                   # an ATX heading begins the line\n    |   [ ]{,3}<(            # all types of HTML blocks except type 7 may interrupt a paragraph\n          (?xi:\n  (script|style|pre)\\b\n)   # 1\n        | !--                                   # 2\n        | \\?                                    # 3\n        | ![A-Z]                                # 4\n        | !\\[CDATA\\[                            # 5\n        | (?x:\n  /?\n  (?i:address|article|aside|base|basefont|blockquote|body|caption|center|col|colgroup|dd|details|dialog|dir|div|dl|dt|fieldset|figcaption|figure|footer|form|frame|frameset|h1|h2|h3|h4|h5|h6|head|header|hr|html|iframe|legend|li|link|main|menu|menuitem|meta|nav|noframes|ol|optgroup|option|p|param|section|source|summary|table|tbody|td|tfoot|th|thead|title|tr|track|ul)\n  (?:\\s|(?m:$)|/?>)\n)  # 6\n        )\n    )\n)"),
      scope: vec![],
      captures: None,
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Include(ContextReference::Direct(ContextId { index: 3339 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 2107 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(={3,})(?=[ \\t]*(?m:$))"),
      scope: vec![
        Scope {
            a: 114281636629382074,
            b: 13792273858822144,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629353710522,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^(-{3,})(?=[ \\t]*(?m:$))"),
      scope: vec![
        Scope {
            a: 114281636568695738,
            b: 13792273858822144,
        },
    ],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288629353710522,
            b: 13792273858822144,
        },
    ])]),
      operation: MatchOperation::Pop,
      with_prototype: None
    }),
]
} }