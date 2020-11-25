
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
      regex: Regex::new("(?x)\\bon(\n  abort|autocomplete|autocompleteerror|auxclick|blur|cancel|canplay\n  |canplaythrough|change|click|close|contextmenu|cuechange|dblclick|drag\n  |dragend|dragenter|dragexit|dragleave|dragover|dragstart|drop\n  |durationchange|emptied|ended|error|focus|input|invalid|keydown\n  |keypress|keyup|load|loadeddata|loadedmetadata|loadstart|mousedown\n  |mouseenter|mouseleave|mousemove|mouseout|mouseover|mouseup|mousewheel\n  |pause|play|playing|progress|ratechange|reset|resize|scroll|seeked\n  |seeking|select|show|sort|stalled|submit|suspend|timeupdate|toggle\n  |volumechange|waiting\n)\\b"),
      scope: vec![
        Scope {
            a: 59392186477183220,
            b: 1407374883553280,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 10176 }),
        ContextReference::Direct(ContextId { index: 10175 }),
    ]),
      with_prototype: None
    }),
]
} }