
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
      regex: Regex::new("\\b(absolute|all(-scroll)?|always|armenian|auto|avoid|baseline|below|bidi-override|block|bold(er)?|(border|content|padding)-box|both|bottom|break-all|break-word|capitalize|center|char|circle|cjk-ideographic|col-resize|collapse|crosshair|cursive|dashed|decimal-leading-zero|decimal|default|disabled|disc|distribute-all-lines|distribute-letter|distribute-space|distribute|dotted|double|e-resize|ellipsis|fantasy|fixed|geometricPrecision|georgian|groove|hand|hebrew|help|hidden|hiragana-iroha|hiragana|horizontal|ideograph-alpha|ideograph-numeric|ideograph-parenthesis|ideograph-space|inactive|inherit|inline-block|inline|inset|inside|inter-ideograph|inter-word|italic|justify|katakana-iroha|katakana|keep-all|left|lighter|line-edge|line-through|line|list-item|loose|lower-alpha|lower-greek|lower-latin|lower-roman|lowercase|lr-tb|ltr|medium|middle|move|monospace|n-resize|ne-resize|newspaper|no-drop|no-repeat|nw-resize|none|normal|not-allowed|nowrap|oblique|optimize(Legibility|Quality|Speed)|outset|outside|overline|pointer|pre(-(wrap|line))?|progress|relative|repeat-x|repeat-y|repeat|right|ridge|row-resize|rtl|(sans-)?serif|s-resize|scroll|se-resize|separate|small-caps|solid|square|static|strict|sub|super|sw-resize|table(-(row|cell|footer-group|header-group))?|tb-rl|text-bottom|text-top|text|thick|thin|top|transparent|underline|upper-alpha|upper-latin|upper-roman|uppercase|vertical(-(ideographic|text))?|visible(Painted|Fill|Stroke)?|w-resize|wait|whitespace|zero|smaller|larger|((xx?-)?(small(er)?|large(r)?))|painted|fill|stroke)\\b"),
      scope: vec![
        Scope {
            a: 59956308942585856,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }