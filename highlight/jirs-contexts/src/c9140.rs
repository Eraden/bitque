
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
      regex: Regex::new("\\b(zip|variable-exists|unquote|unitless|unit|unique-id|type-of|transparentize|to-upper-case|to-lower-case|tan|str-slice|str-length|str-insert|str-index|sqrt|spin|softlight|sin|simple-selectors|set-nth|selector-unify|selector-replace|selector-parse|selector-nest|selector-extend|selector-append|screen|scale-color|saturation|saturate|round|rgba|rgb|replace|red|random|quote|pow|pi|percentage|overlay|opacify|nth|negation|multiply|mod|mixin-exists|mix|min|max|map-values|map-remove|map-merge|map-keys|map-has-key|map-get|luma|list-separator|lightness|lighten|length|keywords|join|isnumber|is-superselector|is-bracketed|invert|inspect|index|if|ie-hex-str|hue|hsvvalue|hsvsaturation|hsvhue|hsva|hsv|hsla|hsl|hardlight|greyscale|green|grayscale|global-variable-exists|get-function|function-exists|format|floor|feature-exists|fadeout|fadein|fade|extract|exclusion|escape|e|difference|desaturate|data-uri|darken|cos|convert|contrast|content-exists|complement|comparable|color|change-color|ceil|call|calc|blue|average|atan|asin|argb|append|alpha|adjust-hue|adjust-color|acos|abs)(?=\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925255094796288,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9057 }),
    ]),
      with_prototype: None
    }),
]
} }