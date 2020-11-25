
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
      regex: Regex::new("\\b(trintreeshape|trintreepath|treeviewer|treeshape|treepath|time2date|swaptionbyhw|swaptionbyhjm|swaptionbybk|swaptionbybdt|swapbyzero|swapbyhw|swapbyhjm|swapbybk|swapbybdt|stockspec|stockoptspec|ratetimes|rate2disc|optstockbyitt|optstockbyeqp|optstockbycrr|optbndbyhw|optbndbyhjm|optbndbybk|optbndbybdt|mmktbyhjm|mmktbybdt|mktrintree|mktree|mkbush|lookbackbyitt|lookbackbyeqp|lookbackbycrr|itttree|itttimespec|ittsens|ittprice|isafin|intenvset|intenvsens|intenvprice|intenvget|insttypes|instswaption|instswap|instsetfield|instselect|instoptstock|instoptbnd|instlookback|instlength|instgetcell|instget|instfloor|instfloat|instfixed|instfind|instfields|instdisp|instdelete|instcompound|instcf|instcap|instbond|instbarrier|instasian|instaddfield|instadd|hwvolspec|hwtree|hwtimespec|hwsens|hwprice|hjmvolspec|hjmtree|hjmtimespec|hjmsens|hjmprice|hedgeslf|hedgeopt|floorbyhw|floorbyhjm|floorbybk|floorbybdt|floatbyzero|floatbyhw|floatbyhjm|floatbybk|floatbybdt|fixedbyzero|fixedbyhw|fixedbyhjm|fixedbybk|fixedbybdt|eqptree|eqptimespec|eqpsens|eqpprice|disc2rate|derivset|derivget|datedisp|date2time|cvtree|crrtree|crrtimespec|crrsens|crrprice|compoundbyitt|compoundbyeqp|compoundbycrr|classfin|cfbyzero|cfbyhw|cfbyhjm|cfbybk|cfbybdt|capbyhw|capbyhjm|capbybk|capbybdt|bushshape|bushpath|bondbyzero|bondbyhw|bondbyhjm|bondbybk|bondbybdt|bkvolspec|bktree|bktimespec|bksens|bkprice|bdtvolspec|bdttree|bdttimespec|bdtsens|bdtprice|barrierbyitt|barrierbyeqp|barrierbycrr|asianbyitt|asianbyeqp|asianbycrr)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044133,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }