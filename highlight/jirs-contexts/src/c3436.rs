
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
      regex: Regex::new("\\b(modelinput|getAlternativeTypes|getAlternativeNames|YData|XDataNames|XData|Widths|Values|Value|UserVariables|UpdateResponseFeatures|UpdateResponse|Units|Type|TestPlans|TestFilters|SummaryStatisticsForTest|SummaryStatistics|StepwiseStatus|StepwiseSelection|StepwiseRegression|Status|StatisticsDialog|SizeOfParameterSet|SingleVIF|SignalUnits|SignalNames|SetupDialog|SetTermStatus|SaveAs|Save|RollbackEdit|RestoreDataForTest|RestoreData|Responses|ResponseSignalName|Response|RemoveVariable|RemoveTestFilter|RemoveOutliersForTest|RemoveOutliers|RemoveFilter|RemoveData|Remove|RecordsPerTest|Properties|PredictedValueForTest|PredictedValue|PartialVIF|Parameters|ParameterStatistics|PEVForTest|PEV|Owner|OutputData|OutlierIndicesForTest|OutlierIndices|NumberOfTests|NumberOfRecords|NumberOfParameters|NumberOfInputs|New|Names|Name|MultipleVIF|ModifyVariable|ModifyTestFilter|ModifyFilter|Modified|ModelSetup|ModelForTest|Model|MakeHierarchicalResponse|LocalResponses|LoadProject|Load|Levels|Level|Jacobian|IsEditable|IsBeingEdited|IsAlternative|InputsPerLevel|Inputs|InputSignalNames|InputSetupDialog|InputData|ImportFromMBCDataStructure|ImportFromFile|GetTermStatus|GetTermLabel|GetIncludedTerms|GetDesignMatrix|GetAllTerms|FitAlgorithm|Fit|Filters|Filename|ExportToMBCDataStructure|Export|Evaluate|DoubleResponseData|DoubleInputData|DiagnosticStatistics|DetachData|DefineTestGroups|DefineNumberOfRecordsPerTest|DefaultModels|DataFileTypes|Data|CreateTestplan|CreateResponseFeature|CreateResponse|CreateProject|CreateModel|CreateData|CreateAlternativeModels|CreateAlgorithm|Covariance|Correlation|CopyData|CommitEdit|ChooseAsBest|Centers|BoxCoxSSE|BeginEdit|AttachData|Append|AlternativeResponses|AlternativeModelStatistics|AliasMatrix|AddVariable|AddTestFilter|AddFilter)\\b"),
      scope: vec![
        Scope {
            a: 61925255150044142,
            b: 14355223812243456,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }