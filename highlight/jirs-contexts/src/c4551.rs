
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
      regex: Regex::new("(?x)\\b(\n\t__import__|all|abs|any|apply|ascii|bin|breakpoint|callable|chr|classmethod|cmp|coerce|\n\tcompile|delattr|dir|divmod|enumerate|eval|exec|execfile|filter|format|getattr|\n\tglobals|hasattr|hash|help|hex|id|input|intern|isinstance|issubclass|iter|\n\tlen|locals|map|max|min|next|oct|open|ord|pow|print|property|range|\n\traw_input|reduce|reload|repr|reversed|round|setattr|sorted|staticmethod|\n\tsum|super|type|unichr|vars|zip\n)\\b"),
      scope: vec![
        Scope {
            a: 61925255090602046,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }