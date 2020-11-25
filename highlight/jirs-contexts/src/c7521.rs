
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
      regex: Regex::new("(?x) # ignore whitespace in this regex\n  \\b(\n    abs |\n    acos |\n    all |\n    any |\n    asin |\n    atan |\n    ceil |\n    clamp |\n    cos |\n    cross |\n    degrees |\n    dFdx |\n    dFdy |\n    distance |\n    dot |\n    equal |\n    exp |\n    exp2 |\n    faceforward |\n    floor |\n    fract |\n    ftransform |\n    fwidth |\n    greaterThan |\n    greaterThanEqual |\n    inversesqrt |\n    length |\n    lessThan |\n    lessThanEqual |\n    log |\n    log2 |\n    matrixCompMult |\n    max |\n    min |\n    mix |\n    mod |\n    noise[1-4] |\n    normalize |\n    not |\n    notEqual |\n    outerProduct |\n    pow |\n    radians |\n    reflect |\n    refract |\n    shadow1D |\n    shadow1DLod |\n    shadow1DProj |\n    shadow1DProjLod |\n    shadow2D |\n    shadow2DLod |\n    shadow2DProj |\n    shadow2DProjLod |\n    sign |\n    sin |\n    smoothstep |\n    sqrt |\n    step |\n    tan |\n    texture |\n    texture1D |\n    texture1DLod |\n    texture1DProj |\n    texture1DProjLod |\n    texture2D |\n    texture2DLod |\n    texture2DProj |\n    texture2DProjLod |\n    texture3D |\n    texture3DLod |\n    texture3DProj |\n    texture3DProjLod |\n    textureCube |\n    textureCubeLod |\n    transpose\n  )\\b\n"),
      scope: vec![
        Scope {
            a: 61925255092961280,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }