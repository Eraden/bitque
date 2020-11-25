
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
      regex: Regex::new("\\b(ztrans|zeta|vpa|uint8|uint64|uint32|uint16|triu|tril|taylortool|taylor|symsum|syms|sym2poly|sym|svd|subs|subexpr|sort|solve|size|sinint|single|simplify|simple|rsums|rref|round|real|rank|quorem|procread|pretty|poly2sym|poly|numden|null|mod|mhelp|mfunlist|mfun|mapleinit|maple|log2|log10|limit|latex|laplace|lambertw|jordan|jacobian|iztrans|inv|int8|int64|int32|int16|int|imag|ilaplace|ifourier|hypergeom|horner|heaviside|funtool|frac|fourier|fortran|floor|fix|finverse|findsym|factor|ezsurfc|ezsurf|ezpolar|ezplot3|ezplot|ezmeshc|ezmesh|ezcontourf|ezcontour|expm|expand|eq|eig|dsolve|double|dirac|digits|diff|diag|det|cosint|conj|compose|colspace|collect|coeffs|ceil|ccode)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044152,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }