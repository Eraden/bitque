
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
      regex: Regex::new("\\b(unit|tan|sqrt|spin|softlight|sin|screen|saturation|saturate|round|replace|red|pow|pi|percentage|overlay|negation|multiply|mod|mix|min|max|luma|lightness|lighten|length|isnumber|hue|hsvvalue|hsvsaturation|hsvhue|hsva|hsv|hsla|hsl|hardlight|greyscale|green|format|floor|fadeout|fadein|fade|extract|exclusion|escape|e|difference|desaturate|data-uri|darken|cos|convert|convert|contrast|color|ceil|calc|blue|average|atan|asin|argb|alpha|acos|abs)(\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255093616640,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46443865086558208,
            b: 0,
        },
        Scope {
            a: 47288629318582454,
            b: 34621422135410688,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8211 }),
    ]),
      with_prototype: None
    }),
]
} }