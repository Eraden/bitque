
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
      regex: Regex::new("\\b(alim|allchild|alpha|alphamap|ancestor|annotation|area|axes|axis|bar|bar3|bar3h|barh|box|brighten|camdolly|cameratoolbar|camlight|camlookat|camorbit|campan|campos|camproj|camroll|camtarget|camup|camva|camzoom|caxis|cla|clabel|clf|close|closereq|colorbar|colordef|colormap|colormapeditor|ColorSpec|comet|comet3|compass|coneplot|contour|contour3|contourc|contourf|contourslice|contrast|copyobj|curl|cylinder|daspect|datacursormode|datetick|delaunay|delaunay3|delaunayn|delete|diffuse|divergence|dragrect|drawnow|dsearch|dsearchn|ellipsoid|errorbar|ezcontour|ezcontourf|ezmesh|ezmeshc|ezplot|ezplot3|ezpolar|ezsurf|ezsurfc|feather|figure|figurepalette|fill|fill3|findall|findfigs|findobj|flow|fplot|frame2im|frameedit|gca|gcbf|gcbo|gcf|gco|get|getframe|graymon|grid|gtext|hgexport|hggroup|hgload|hgsave|hgtransform|hidden|hist|histc|hold|hsv2rgb|im2frame|im2java|image|imagesc|imformats|ind2rgb|inpolygon|interpstreamspeed|isocaps|isocolors|isonormals|isosurface|legend|light|lightangle|lighting|line|LineSpec|linkaxes|linkprop|loglog|makehgtform|material|mesh|meshc|meshz|movie|newplot|noanimate|opengl|orient|pan|pareto|patch|pbaspect|pcolor|peaks|pie|pie3|plot|plot3|plotbrowser|plotedit|plotmatrix|plottools|plotyy|polar|polyarea|print|printopt|propedit|propertyeditor|quiver|quiver3|rbbox|rectangle|rectint|reducepatch|reducevolume|refresh|refreshdata|reset|rgb2hsv|rgbplot|ribbon|rose|rotate|rotate3d|saveas|scatter|scatter3|semilogx|semilogy|set|shading|showplottool|shrinkfaces|slice|smooth3|specular|sphere|spinmap|stairs|stem|stem3|stream2|stream3|streamline|streamparticles|streamribbon|streamslice|streamtube|subplot|subvolume|surf|surf2patch|surface|surfc|surfl|surfnorm|tetramesh|texlabel|text|title|trimesh|triplot|trisurf|tsearch|tsearchn|view|viewmtx|volumebounds|voronoi|voronoin|waterfall|whitebg|xlabel|xlim|ylabel|ylim|zlabel|zlim|zoom)\\b"),
      scope: vec![
        Scope {
            a: 61925255149977651,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }