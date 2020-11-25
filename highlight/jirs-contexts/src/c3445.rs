
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
      regex: Regex::new("\\b(ztest|zscore|x2fx|wishrnd|wblstat|wblrnd|wblplot|wblpdf|wbllike|wblinv|wblfit|wblcdf|view|vartestn|vartest2|vartest|var|upperparams|unifstat|unifrnd|unifpdf|unifit|unifinv|unifcdf|unidstat|unidrnd|unidpdf|unidinv|unidcdf|type|ttest2|ttest|tstat|trnd|trimmean|treeval|treetest|treeprune|treefit|treedisp|tpdf|tinv|tiedrank|test|tdfread|tcdf|tblwrite|tblread|tabulate|surfht|summary|stepwisefit|stepwise|std|statset|statget|squareform|sortrows|sort|slicesample|skewness|silhouette|signtest|signrank|setlabels|set|segment|scatterhist|sampsizepwr|runstest|rstool|rsmdemo|rowexch|rotatefactors|robustfit|robustdemo|risk|ridge|replacedata|reorderlevels|regstats|regress|refline|refcurve|rcoplot|raylstat|raylrnd|raylpdf|raylinv|raylfit|raylcdf|ranksum|range|randtool|randsample|random|randg|quantile|qqplot|prune|procrustes|probplot|princomp|prctile|posterior|polyval|polytool|polyfit|polyconf|poisstat|poissrnd|poisspdf|poissinv|poissfit|poisscdf|perms|pearsrnd|pdist|pdf|pcares|pcacov|partialcorr|paretotails|pareto|parent|parallelcoords|ordinal|numnodes|nsegments|normstat|normspec|normrnd|normplot|normpdf|normlike|norminv|normfit|normcdf|nominal|nodesize|nodeprob|nodeerr|nlpredci|nlparci|nlintool|nlinfit|ncx2stat|ncx2rnd|ncx2pdf|ncx2inv|ncx2cdf|nctstat|nctrnd|nctpdf|nctinv|nctcdf|ncfstat|ncfrnd|ncfpdf|ncfinv|ncfcdf|nbinstat|nbinrnd|nbinpdf|nbininv|nbinfit|nbincdf|nanvar|nansum|nanstd|nanmin|nanmedian|nanmean|nanmax|nancov|mvtrnd|mvtpdf|mvtcdf|mvregresslike|mvregress|mvnrnd|mvnpdf|mvncdf|multivarichart|multcompare|moment|mode|mnrval|mnrnd|mnrfit|mnpdf|mlecov|mle|mhsample|mergelevels|median|mean|mdscale|manovacluster|manova1|maineffectsplot|mahal|mad|lsqnonneg|lsline|lscov|lowerparams|lognstat|lognrnd|lognpdf|lognlike|logninv|lognfit|logncdf|linkage|linhyptest|lillietest|lhsnorm|lhsdesign|leverage|levelcounts|kurtosis|kstest2|kstest|ksdensity|kruskalwallis|kmeans|join|johnsrnd|jbtest|jackknife|iwishrnd|isundefined|ismember|islevel|isbranch|iqr|invpred|interactionplot|inconsistent|icdf|hygestat|hygernd|hygepdf|hygeinv|hygecdf|hougen|hmmviterbi|hmmtrain|hmmgenerate|hmmestimate|hmmdecode|histfit|hist3|hist|harmmean|hadamard|gscatter|grpstats|grp2idx|gpstat|gprnd|gppdf|gplotmatrix|gplike|gpinv|gpfit|gpcdf|gname|gmdistribution|glyphplot|glmval|glmfit|gline|gevstat|gevrnd|gevpdf|gevlike|gevinv|gevfit|gevcdf|getlabels|get|geostat|geornd|geopdf|geomean|geoinv|geocdf|gamstat|gamrnd|gampdf|gamlike|gaminv|gamfit|gamcdf|gagerr|fullfact|fsurfht|fstat|frnd|friedman|fracfactgen|fracfact|fpdf|fit|finv|ff2n|fcdf|factoran|expstat|exprnd|exppdf|explike|expinv|expfit|expcdf|evstat|evrnd|evpdf|evlike|evinv|evfit|evcdf|eval|errorbar|ecdfhist|ecdf|dwtest|dummyvar|droplevels|disttool|dfittool|dendrogram|dcovary|daugment|datasetfun|dataset|cutvar|cuttype|cutpoint|cutcategories|crosstab|coxphfit|cov|corrcov|corrcoef|corr|cordexch|copulastat|copularnd|copulapdf|copulaparam|copulafit|copulacdf|cophenet|controlrules|controlchart|combnk|cmdscale|clusterdata|cluster|classregtree|classprob|classify|classcount|cholcov|children|chi2stat|chi2rnd|chi2pdf|chi2inv|chi2gof|chi2cdf|cdfplot|cdf|ccdesign|casewrite|caseread|capaplot|capability|canoncorr|candgen|candexch|boxplot|boundary|bootstrp|bootci|biplot|binostat|binornd|binopdf|binoinv|binofit|binocdf|betastat|betarnd|betapdf|betalike|betainv|betafit|betacdf|bbdesign|barttest|aoctool|ansaribradley|anovan|anova2|anova1|andrewsplot|addlevels|addedvarplot)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044151,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }