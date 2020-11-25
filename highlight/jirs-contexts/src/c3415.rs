
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
      regex: Regex::new("\\b(zonebackadj|weights|view|traverse|traceplot|topoorder|swalign|svmtrain|svmsmoset|svmclassify|subtree|sptread|showhmmprof|showalignment|shortestpath|seqwordcount|seqtool|seqshowwords|seqshoworfs|seqreverse|seqrcomplement|seqprofile|seqpdist|seqneighjoin|seqmatch|seqlogo|seqlinkage|seqinsertgaps|seqdotplot|seqdisp|seqconsensus|seqcomplement|seq2regexp|select|scfread|samplealign|rnaplot|rnafold|rnaconvert|rna2dna|rmasummary|rmabackadj|revgeneticcode|restrict|reroot|reorder|redgreencmap|rebasecuts|rankfeatures|randseq|randfeatures|ramachandran|quantilenorm|prune|proteinpropplot|proteinplot|profalign|probesetvalues|probesetplot|probesetlookup|probesetlink|probelibraryinfo|plot|phytreewrite|phytreetool|phytreeread|phytree|pfamhmmread|pdist|pdbwrite|pdbread|pdbdistplot|pam|palindromes|optimalleaforder|oligoprop|nwalign|num2goid|nuc44|ntdensity|nt2int|nt2aa|nmercount|mzxmlread|mzxml2peaks|multialignviewer|multialignread|multialign|msviewer|mssgolay|msresample|msppresample|mspeaks|mspalign|msnorm|mslowess|msheatmap|msdotplot|msbackadj|msalign|molweight|molviewer|minspantree|maxflow|mavolcanoplot|mattest|mapcaplot|manorm|malowess|maloglog|mairplot|mainvarsetnorm|maimage|magetfield|mafdr|maboxplot|knnimpute|knnclassify|joinseq|jcampread|isspantree|isomorphism|isoelectric|isdag|int2nt|int2aa|imageneread|hmmprofstruct|hmmprofmerge|hmmprofgenerate|hmmprofestimate|hmmprofalign|graphtraverse|graphtopoorder|graphshortestpath|graphpred2path|graphminspantree|graphmaxflow|graphisspantree|graphisomorphism|graphisdag|graphconncomp|graphcluster|graphallshortestpaths|gprread|gonnet|goannotread|getrelatives|getpdb|getnodesbyid|getnewickstr|getmatrix|gethmmtree|gethmmprof|gethmmalignment|getgeodata|getgenpept|getgenbank|getembl|getedgesbynodeid|getdescendants|getcanonical|getbyname|getblast|getancestors|get|geosoftread|genpeptread|genevarfilter|geneticcode|generangefilter|geneont|genelowvalfilter|geneentropyfilter|genbankread|gcrmabackadj|gcrma|galread|featuresparse|featuresmap|fastawrite|fastaread|exprprofvar|exprprofrange|evalrasmolscript|emblread|dolayout|dndsml|dnds|dna2rna|dimercount|dayhoff|cytobandread|crossvalind|cpgisland|conncomp|codoncount|codonbias|clustergram|cleave|classperf|chromosomeplot|cghcbs|celintensityread|blosum|blastreadlocal|blastread|blastncbi|blastlocal|blastformat|biograph|baselookup|basecount|atomiccomp|aminolookup|allshortestpaths|agferead|affyread|affyprobeseqread|affyprobeaffinities|affyinvarsetnorm|aacount|aa2nt|aa2int)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044122,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }