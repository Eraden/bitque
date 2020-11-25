
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
      regex: Regex::new("\\b((?x) # ignore whitespace in this regex\n  \\b(\n    layout |\n    attribute |\n    centroid |\n    sampler |\n    patch |\n    const |\n    flat |\n    in |\n    inout |\n    invariant |\n    noperspective |\n    out |\n    smooth |\n    uniform |\n    varying |\n    buffer |\n    shared |\n    coherent |\n    readonly |\n    writeonly |\n    volatile |\n    restrict\n  )\\b\n)\\b"),
      scope: vec![
        Scope {
            a: 48414439030980608,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }