
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
      regex: Regex::new("(?x) # ignore whitespace in this regex\n  \\bgl_(\n    BackColor |\n    BackLightModelProduct |\n    BackLightProduct |\n    BackMaterial |\n    BackSecondaryColor |\n    ClipDistance |\n    ClipPlane |\n    ClipVertex |\n    Color |\n    DepthRange |\n    DepthRangeParameters |\n    EyePlaneQ |\n    EyePlaneR |\n    EyePlaneS |\n    EyePlaneT |\n    Fog |\n    FogCoord |\n    FogFragCoord |\n    FogParameters |\n    FragColor |\n    FragCoord |\n    FragDat |\n    FragDept |\n    FrontColor |\n    FrontFacing |\n    FrontLightModelProduct |\n    FrontLightProduct |\n    FrontMaterial |\n    FrontSecondaryColor |\n    InstanceID |\n    Layer |\n    LightModel |\n    LightModelParameters |\n    LightModelProducts |\n    LightProducts |\n    LightSource |\n    LightSourceParameters |\n    MaterialParameters |\n    ModelViewMatrix |\n    ModelViewMatrixInverse |\n    ModelViewMatrixInverseTranspose |\n    ModelViewMatrixTranspose |\n    ModelViewProjectionMatrix |\n    ModelViewProjectionMatrixInverse |\n    ModelViewProjectionMatrixInverseTranspose |\n    ModelViewProjectionMatrixTranspose |\n    MultiTexCoord[0-7] |\n    Normal |\n    NormalMatrix |\n    NormalScale |\n    ObjectPlaneQ |\n    ObjectPlaneR |\n    ObjectPlaneS |\n    ObjectPlaneT |\n    Point |\n    PointCoord |\n    PointParameters |\n    PointSize |\n    Position |\n    PrimitiveIDIn |\n    ProjectionMatrix |\n    ProjectionMatrixInverse |\n    ProjectionMatrixInverseTranspose |\n    ProjectionMatrixTranspose |\n    SecondaryColor |\n    TexCoord |\n    TextureEnvColor |\n    TextureMatrix |\n    TextureMatrixInverse |\n    TextureMatrixInverseTranspose |\n    TextureMatrixTranspose |\n    Vertex |\n    VertexID\n  )\\b\n"),
      scope: vec![
        Scope {
            a: 49259061529608192,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }