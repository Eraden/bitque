
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
      regex: Regex::new("\\b(wrldmagm|updateNodes|updateCamera|updateBodies|update|show|saveas|rrtheta|rrsigma|rrdelta|removeViewpoint|removeNode|removeBody|read|quatrotate|quatnormalize|quatnorm|quatmultiply|quatmod|quatinv|quatdivide|quatconj|quat2dcm|quat2angle|play|nodeInfo|moveBody|move|mjuliandate|machnumber|load|lla2ecef|leapyear|juliandate|initialize|initIfNeeded|hide|gravitywgs84|geoidegm96|geod2geoc|geocradius|geoc2geod|generatePatches|findstartstoptimes|fganimation|ecef2lla|dpressure|delete|decyear|dcmecef2ned|dcmbody2wind|dcm2quat|dcm2latlon|dcm2angle|dcm2alphabeta|datcomimport|createBody|correctairspeed|convvel|convtemp|convpres|convmass|convlength|convforce|convdensity|convangvel|convangacc|convang|convacc|atmospalt|atmosnrlmsise00|atmosnonstd|atmoslapse|atmosisa|atmoscoesa|atmoscira|angle2quat|angle2dcm|alphabeta|airspeed|addViewpoint|addRoute|addNode|addBody|VirtualRealityAnimation|Viewpoint|Node|Geometry|GenerateRunScript|Camera|Body|Animation)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044121,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }