
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
      regex: Regex::new("(?x) # ignore whitespace in this regex\n  \\bgl_(\n    MaxClipPlanes |\n    MaxCombinedTextureImageUnits |\n    MaxDrawBuffers |\n    MaxFragmentUniformComponents |\n    MaxLights |\n    MaxTextureCoords |\n    MaxTextureImageUnits |\n    MaxTextureUnits |\n    MaxVaryingFloats |\n    MaxVertexAttribs |\n    MaxVertexTextureImageUnits |\n    MaxVertexUniformComponents\n  )\\b\n"),
      scope: vec![
        Scope {
            a: 61925409711783936,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }