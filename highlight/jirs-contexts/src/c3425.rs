
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
      regex: Regex::new("\\b(zero2pyld|zero2fwd|zero2disc|zbtyield|zbtprice|yldtbill|yldmat|ylddisc|yearfrac|yeardays|year|xirr|x2mdate|wrkdydif|willpctr|willad|weights2holdings|weekday|wclose|volroc|vertcat|uplus|uminus|uicalendar|ugarchsim|ugarchpred|ugarchllf|ugarch|typprice|tsmovavg|tsmom|tsaccel|tr2bonds|toweekly|totalreturnprice|tosemi|toquoted|toquarterly|tomonthly|todecimal|today|todaily|toannual|times|time2date|tick2ret|thirtytwo2dec|thirdwednesday|tbl2bond|taxedrr|targetreturn|subsref|subsasgn|stochosc|std|spctkd|sortfts|smoothts|size|sharpe|setfield|selectreturn|second|rsindex|rmfield|ret2tick|resamplets|rdivide|pyld2zero|pvvar|pvtrend|pvfix|prtbill|prmat|prdisc|prcroc|prbyzero|power|posvolidx|portvrisk|portstats|portsim|portrand|portopt|portcons|portalpha|portalloc|pointfig|plus|plot|periodicreturns|peravg|pcpval|pcglims|pcgcomp|pcalims|payuni|payper|payodd|payadv|opprofit|onbalvol|nweekdate|now|nomrr|negvolidx|mvnrstd|mvnrobj|mvnrmle|mvnrfish|mtimes|mrdivide|movavg|months|month|mirr|minute|minus|min|merge|medprice|mean|maxdrawdown|max|macd|m2xdate|lweekdate|lpm|log2|log10|log|llow|length|leadts|lbusdate|lagts|issorted|isfield|isequal|iscompatible|isbusday|irr|inforatio|hour|horzcat|holidays|holdings2weights|hist|highlow|hhigh|getnameidx|getfield|geom2arith|fwd2zero|fvvar|fvfix|fvdisc|ftsuniq|ftstool|ftsinfo|ftsgui|ftsbound|fts2mat|fts2ascii|frontier|frontcon|freqstr|freqnum|frac2cur|fpctkd|fints|filter|fillts|fieldnames|fetch|fbusdate|extfield|exp|ewstats|eomday|eomdate|end|emaxdrawdown|elpm|effrr|ecmnstd|ecmnobj|ecmnmle|ecmninit|ecmnhess|ecmnfish|ecmmvnrstd|ecmmvnrobj|ecmmvnrmle|ecmmvnrfish|ecmlsrobj|ecmlsrmle|discrate|disc2zero|diff|depstln|depsoyd|deprdv|depgendb|depfixdb|dec2thirtytwo|daysdif|daysadd|daysact|days365|days360psa|days360isda|days360e|days360|day|datewrkdy|datevec|datestr|datenum|datemnth|datefind|datedisp|dateaxis|date2time|cur2str|cur2frac|cumsum|createholidays|cpnpersz|cpndaysp|cpndaysn|cpndatepq|cpndatep|cpndatenq|cpndaten|cpncount|cov2corr|corr2cov|convertto|convert2sur|chfield|chartfts|chaikvolat|chaikosc|cftimes|cfport|cfdur|cfdates|cfconv|cfamounts|candle|busdays|busdate|boxcox|bollinger|bolling|bndyield|bndspread|bndprice|bnddury|bnddurp|bndconvy|bndconvp|blsvega|blstheta|blsrho|blsprice|blslambda|blsimpv|blsgamma|blsdelta|blkprice|blkimpv|binprice|beytbill|barh|bar3h|bar3|bar|ascii2fts|arith2geom|annuterm|annurate|amortize|adosc|adline|active2abs|acrudisc|acrubond|accrfrac|abs2active)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044132,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }