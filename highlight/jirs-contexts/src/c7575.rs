
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
      regex: Regex::new("\\b((?x) # ignore whitespace in this regex\n  \\b(\n    void |\n    bool |\n    int |\n    uint |\n    float |\n    double |\n    vec[2-4] |\n    dvec[2-4] |\n    bvec[2-4] |\n    ivec[2-4] |\n    uvec[2-4] |\n    mat[2-4] |\n    mat2x2 |\n    mat2x3 |\n    mat2x4 |\n    mat3x2 |\n    mat3x3 |\n    mat3x4 |\n    mat4x2 |\n    mat4x3 |\n    mat4x4 |\n    dmat2 |\n    dmat3 |\n    dmat4 |\n    dmat2x2 |\n    dmat2x3 |\n    dmat2x4 |\n    dmat3x2 |\n    dmat3x3 |\n    dmat3x4 |\n    dmat4x2 |\n    dmat4x3 |\n    dmat4x4 |\n    sampler[1-3]D |\n    image[1-3]D |\n    samplerCube |\n    imageCube |\n    sampler2DRect |\n    image2DRect |\n    sampler[12]DArray |\n    image[12]DArray |\n    samplerBuffer |\n    imageBuffer |\n    sampler2DMS |\n    image2DMS |\n    sampler2DMSArray |\n    image2DMSArray |\n    samplerCubeArray |\n    imageCubeArray |\n    sampler[12]DShadow |\n    sampler2DRectShadow |\n    sampler[12]DArrayShadow |\n    samplerCubeShadow |\n    samplerCubeArrayShadow |\n    isampler[1-3]D |\n    iimage[1-3]D |\n    isamplerCube |\n    iimageCube |\n    isampler2DRect |\n    iimage2DRect |\n    isampler[12]DArray |\n    iimage[12]DArray |\n    isamplerBuffer |\n    iimageBuffer |\n    isampler2DMS |\n    iimage2DMS |\n    isampler2DMSArray |\n    iimage2DMSArray |\n    isamplerCubeArray |\n    iimageCubeArray |\n    atomic_uint |\n    usampler[1-3]D |\n    uimage[1-3]D |\n    usamplerCube |\n    uimageCube |\n    usampler2DRect |\n    uimage2DRect |\n    usampler[12]DArray |\n    uimage[12]DArray |\n    usamplerBuffer |\n    uimageBuffer |\n    usampler2DMS |\n    uimage2DMS |\n    usampler2DMSArray |\n    uimage2DMSArray |\n    usamplerCubeArray |\n    uimageCubeArray\n  )\\b\n|struct)\\b"),
      scope: vec![
        Scope {
            a: 48414576469934080,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }