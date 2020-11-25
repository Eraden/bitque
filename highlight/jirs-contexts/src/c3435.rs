
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
      regex: Regex::new("\\b(zerom|zero22pi|zdatam-ui|zdatam|wrapToPi|wrapTo360|wrapTo2Pi|wrapTo180|worldmap|worldfilewrite|worldfileread|westof|vmap0ui|vmap0rhead|vmap0read|vmap0data|vinvtran|viewshed|vfwdtran|vec2mtx|utmzoneui|utmzone|utmgeoid|usgsdems|usgsdem|usgs24kdem|usamap|updategeostruct|unwrapMultipart|unitstr|unitsratio|undotrim|undoclip|uimaptbx|trimdata|trimcart|trackui|trackg|track2|track1|track|toRadians|toDegrees|tissot|timezone|timedim|time2str|tightmap|tigerp|tigermif|tgrline|textm|tbase|tagm-ui|tagm|symbolm|surfm|surflsrm|surflm|surfdist|surfacem|str2angle|stem3m|stdm|stdist|spzerom|spcread|smoothlong|sm2rad|sm2nm|sm2km|sm2deg|sizem|showm-ui|showm|showaxes|shapewrite|shaperead|shapeinfo|shaderel|setpostn|setm|setltln|seedm|sectorg|sec2hr|sec2hms|sec2hm|sdtsinfo|sdtsdemread|scxsc|scirclui|scircleg|scircle2|scircle1|scatterm|scaleruler|satbath|rsphere|roundn|rotatetext|rotatem|rootlayr|rhxrh|restack|resizem|removeExtraNanSeparators|refvec2mat|refmat2vec|reducem|reckon|readmtx|readfk5|readfields|rcurve|rad2sm|rad2nm|rad2km|rad2dms|rad2dm|rad2deg|quiverm|quiver3m|qrydata|putpole|projlist|projinv|projfwd|project|previewmap|polyxpoly|polysplit|polymerge|polyjoin|polycut|polybool|poly2fv|poly2cw|poly2ccw|polcmap|plotm|plot3m|plabel|pixcenters|pix2map|pix2latlon|pcolorm|patchm|patchesm|parallelui|paperscale|panzoom|originui|org2pol|onem|npi2pi|northarrow|nm2sm|nm2rad|nm2km|nm2deg|newpole|neworig|navfix|nanm|nanclip|namem|n2ecc|mobjects|mlayers|mlabelzero22pi|mlabel|minvtran|minaxis|mfwdtran|meshm|meshlsrm|meshgrat|meridianfwd|meridianarc|meanm|mdistort|mat2hms|mat2dms|mapview|maptrims|maptrimp|maptriml|maptrim|maptool|mapshow|maps|mapprofile|mapoutline|maplist|mapbbox|map2pix|makesymbolspec|makerefmat|makemapped|makedbfspec|makeattribspec|majaxis|lv2ecef|ltln2val|los2|linem|linecirc|limitm|lightmui|lightm|legs|lcolorbar|latlon2pix|kmlwrite|km2sm|km2rad|km2nm|km2deg|ispolycw|ismapped|ismap|isShapeMultipart|intrplon|intrplat|interpm|inputm|ind2rgb8|imbedm|hr2sec|hr2hms|hr2hm|hms2sec|hms2mat|hms2hr|hms2hm|histr|hista|hidem-ui|hidem|handlem-ui|handlem|gtopo30s|gtopo30|gtextm|gshhs|grn2eqa|gridm|grid2image|grepfields|gradientm|globedems|globedem|getworldfilename|getseeds|getm|geotiffread|geotiffinfo|geotiff2mstruct|geoshow|geoloc2grid|geodetic2geocentricLat|geodetic2ecef|geocentric2geodeticLat|gcxsc|gcxgc|gcwaypts|gcpmap|gcm|gc2sc|fromRadians|fromDegrees|framem|flatearthpoly|flat2ecc|fipsname|findm|filterm|fillm|fill3m|extractm|extractfield|etopo5|etopo|eqa2grn|epsm|encodem|ellipse1|elevation|egm96geoid|ecef2lv|ecef2geodetic|ecc2n|ecc2flat|eastof|dteds|dted|driftvel|driftcorr|dreckon|dms2rad|dms2mat|dms2dm|dms2degrees|dms2deg|dm2degrees|distortcalc|distdim|distance|dist2str|displaym|departure|demdataui|demcmap|degrees2dms|degrees2dm|deg2sm|deg2rad|deg2nm|deg2km|deg2dms|deg2dm|defaultm|dcwrhead|dcwread|dcwgaz|dcwdata|daspectm|crossfix|convertlat|contourm|contourfm|contourcmap|contour3m|cometm|comet3m|combntns|colorui|colorm|cmapui|clrmenu|closePolygonParts|clmo-ui|clmo|clma|clipdata|clegendm|clabelm|circcirc|changem|cart2grn|camupm|camtargm|camposm|bufferm|azimuth|axesscale|axesmui|axesm|axes2ecc|avhrrlambert|avhrrgoode|areaquad|areamat|areaint|arcgridread|antipode|angledim|angl2str|almanac)\\b"),
      scope: vec![
        Scope {
            a: 61925255150043697,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }