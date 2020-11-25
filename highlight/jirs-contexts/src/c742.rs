
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
      regex: Regex::new("\\b(hypot(f|l)?|s(scanf|ystem|nprintf|ca(nf|lb(n(f|l)?|ln(f|l)?))|i(n(h(f|l)?|f|l)?|gn(al|bit))|tr(s(tr|pn)|nc(py|at|mp)|c(spn|hr|oll|py|at|mp)|to(imax|d|u(l(l)?|max)|k|f|l(d|l)?)|error|pbrk|ftime|len|rchr|xfrm)|printf|et(jmp|vbuf|locale|buf)|qrt(f|l)?|w(scanf|printf)|rand)|n(e(arbyint(f|l)?|xt(toward(f|l)?|after(f|l)?))|an(f|l)?)|c(s(in(h(f|l)?|f|l)?|qrt(f|l)?)|cos(h(f)?|f|l)?|imag(f|l)?|t(ime|an(h(f|l)?|f|l)?)|o(s(h(f|l)?|f|l)?|nj(f|l)?|pysign(f|l)?)|p(ow(f|l)?|roj(f|l)?)|e(il(f|l)?|xp(f|l)?)|l(o(ck|g(f|l)?)|earerr)|a(sin(h(f|l)?|f|l)?|cos(h(f|l)?|f|l)?|tan(h(f|l)?|f|l)?|lloc|rg(f|l)?|bs(f|l)?)|real(f|l)?|brt(f|l)?)|t(ime|o(upper|lower)|an(h(f|l)?|f|l)?|runc(f|l)?|gamma(f|l)?|mp(nam|file))|i(s(space|n(ormal|an)|cntrl|inf|digit|u(nordered|pper)|p(unct|rint)|finite|w(space|c(ntrl|type)|digit|upper|p(unct|rint)|lower|al(num|pha)|graph|xdigit|blank)|l(ower|ess(equal|greater)?)|al(num|pha)|gr(eater(equal)?|aph)|xdigit|blank)|logb(f|l)?|max(div|abs))|di(v|fftime)|_Exit|unget(c|wc)|p(ow(f|l)?|ut(s|c(har)?|wc(har)?)|error|rintf)|e(rf(c(f|l)?|f|l)?|x(it|p(2(f|l)?|f|l|m1(f|l)?)?))|v(s(scanf|nprintf|canf|printf|w(scanf|printf))|printf|f(scanf|printf|w(scanf|printf))|w(scanf|printf)|a_(start|copy|end|arg))|qsort|f(s(canf|e(tpos|ek))|close|tell|open|dim(f|l)?|p(classify|ut(s|c|w(s|c))|rintf)|e(holdexcept|set(e(nv|xceptflag)|round)|clearexcept|testexcept|of|updateenv|r(aiseexcept|ror)|get(e(nv|xceptflag)|round))|flush|w(scanf|ide|printf|rite)|loor(f|l)?|abs(f|l)?|get(s|c|pos|w(s|c))|re(open|e|ad|xp(f|l)?)|m(in(f|l)?|od(f|l)?|a(f|l|x(f|l)?)?))|l(d(iv|exp(f|l)?)|o(ngjmp|cal(time|econv)|g(1(p(f|l)?|0(f|l)?)|2(f|l)?|f|l|b(f|l)?)?)|abs|l(div|abs|r(int(f|l)?|ound(f|l)?))|r(int(f|l)?|ound(f|l)?)|gamma(f|l)?)|w(scanf|c(s(s(tr|pn)|nc(py|at|mp)|c(spn|hr|oll|py|at|mp)|to(imax|d|u(l(l)?|max)|k|f|l(d|l)?|mbs)|pbrk|ftime|len|r(chr|tombs)|xfrm)|to(b|mb)|rtomb)|printf|mem(set|c(hr|py|mp)|move))|a(s(sert|ctime|in(h(f|l)?|f|l)?)|cos(h(f|l)?|f|l)?|t(o(i|f|l(l)?)|exit|an(h(f|l)?|2(f|l)?|f|l)?)|b(s|ort))|g(et(s|c(har)?|env|wc(har)?)|mtime)|r(int(f|l)?|ound(f|l)?|e(name|alloc|wind|m(ove|quo(f|l)?|ainder(f|l)?))|a(nd|ise))|b(search|towc)|m(odf(f|l)?|em(set|c(hr|py|mp)|move)|ktime|alloc|b(s(init|towcs|rtowcs)|towc|len|r(towc|len))))\\b"),
      scope: vec![
        Scope {
            a: 61925255116226573,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }