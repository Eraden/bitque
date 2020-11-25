
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
      regex: Regex::new("\\b(zlim|ylim|xlim|wordlength|waterfall|voronoin|voronoi|vertcat|upperbound|uplus|uminus|uint8|uint32|uint16|triu|trisurf|triplot|trimesh|tril|treeplot|transpose|tostring|toeplitz|times|text|surfnorm|surfl|surfc|surf|sum|subsref|subsasgn|sub|stripscaling|streamtube|streamslice|streamribbon|stem3|stem|stairs|squeeze|sqrt|spy|slice|size|single|sign|shiftdim|set|semilogy|semilogx|sdec|scatter3|scatter|savefipref|round|rose|ribbon|rgbplot|reshape|resetlog|reset|rescale|repmat|realmin|realmax|real|range|randquant|quiver3|quiver|quantizer|quantize|pow2|polar|plus|plotyy|plotmatrix|plot3|plot|permute|pcolor|patch|or|oct|nunderflows|numerictype|numberofelements|num2int|num2hex|num2bin|noverflows|not|noperations|ne|ndims|mtimes|mpy|minus|minlog|min|meshz|meshc|mesh|maxlog|max|lt|lsb|lowerbound|loglog|logical|line|length|le|isvector|issigned|isscalar|isrow|isreal|ispropequal|isobject|isnumerictype|isnumeric|isnan|isinf|isfinite|isfimath|isfi|isequal|isempty|iscolumn|ipermute|intmin|intmax|int8|int32|int16|int|innerprodintbits|imag|horzcat|histc|hist|hex2num|hex|hankel|gt|gplot|getmsb|getlsb|get|ge|fractionlength|fplot|flipud|fliplr|flipdim|fipref|fimath|fi|feather|ezsurfc|ezsurf|ezpolar|ezplot3|ezplot|ezmesh|ezcontourf|ezcontour|exponentmin|exponentmax|exponentlength|exponentbias|etreeplot|errorbar|eq|eps|end|double|divide|disp|diag|denormalmin|denormalmax|dec|ctranspose|copyobj|convergent|contourf|contourc|contour3|contour|conj|coneplot|complex|compass|comet3|comet|clabel|buffer|bitxorreduce|bitxor|bitsrl|bitsra|bitsll|bitsliceget|bitshift|bitset|bitror|bitrol|bitorreduce|bitor|bitget|bitconcat|bitcmp|bitandreduce|bitand|bin2num|bin|barh|bar|area|any|and|all|add|abs)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044135,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }