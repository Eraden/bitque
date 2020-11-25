
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
      regex: Regex::new("\\b(accumarray|acos|acosd|acosh|acot|acotd|acoth|acsc|acscd|acsch|airy|amd|asec|asecd|asech|asin|asind|asinh|atan|atan2|atand|atanh|balance|besselh|besseli|besselj|besselk|bessely|beta|betainc|betaln|bicg|bicgstab|blkdiag|bsxfun|bvp4c|bvpget|bvpinit|bvpset|bvpxtend|cart2pol|cart2sph|cat|cdf2rdf|ceil|cgs|chol|cholinc|cholupdate|circshift|colamd|colperm|compan|complex|cond|condeig|condest|conj|convhull|convhulln|cos|cosd|cosh|cot|cotd|coth|cross|csc|cscd|csch|cumprod|cumsum|dblquad|dde23|ddeget|ddesd|ddeset|decic|det|deval|diag|disp|display|dmperm|dot|eig|eigs|ellipj|ellipke|erf|erfc|erfcinv|erfcx|erfinv|etree|etreeplot|exp|expint|expm|expm1|eye|factor|factorial|find|fix|flipdim|fliplr|flipud|floor|fminbnd|fminsearch|freqspace|full|funm|fzero|gallery|gamma|gammainc|gammaln|gcd|gmres|gplot|griddata|griddata3|griddatan|gsvd|hadamard|hankel|hess|hilb|horzcat|hypot|idivide|ilu|imag|ind2sub|Inf|inline|interp1|interp1q|interp2|interp3|interpft|interpn|inv|invhilb|ipermute|kron|lcm|ldl|legendre|length|linsolve|linspace|log|log10|log1p|log2|logm|logspace|lscov|lsqnonneg|lsqr|lu|luinc|magic|meshgrid|minres|mkpp|mod|NaN|nchoosek|ndgrid|ndims|nextpow2|nnz|nonzeros|norm|normest|nthroot|null|numel|nzmax|ode113|ode15i|ode15s|ode23|ode23s|ode23t|ode23tb|ode45|odefile|odeget|odeset|odextend|ones|optimget|optimset|ordeig|ordqz|ordschur|orth|pascal|pcg|pchip|pdepe|pdeval|perms|permute|pinv|planerot|pol2cart|poly|polyder|polyeig|polyfit|polyint|polyval|polyvalm|pow2|ppval|primes|prod|psi|qmr|qr|qrdelete|qrinsert|qrupdate|quad|quadl|quadv|qz|rand|randn|randperm|rank|rat|rats|rcond|real|reallog|realpow|realsqrt|rem|repmat|reshape|residue|roots|rosser|rot90|round|rref|rsf2csf|schur|sec|secd|sech|shiftdim|sign|sin|sind|sinh|size|sort|sortrows|spalloc|sparse|spaugment|spconvert|spdiags|speye|spfun|sph2cart|spline|spones|spparms|sprand|sprandn|sprandsym|sprank|spy|sqrt|sqrtm|squeeze|ss2tf|sub2ind|subspace|sum|svd|svds|symamd|symbfact|symmlq|symrcm|tan|tand|tanh|toeplitz|trace|treelayout|treeplot|tril|triplequad|triu|unmkpp|unwrap|vander|vectorize|vertcat|wilkinson|zeros)\\b"),
      scope: vec![
        Scope {
            a: 52640034011152384,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }