
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
      regex: Regex::new("(\\$)(?i:(False|Null|True))\\b"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 59955110645792768,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514563,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)(?i:(Error|ExecutionContext|Host|Home|PID|PsHome|PsVersionTable|ShellID))((?:\\.(?:\\p{L}|\\d|_)+)*\\b)?\\b"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 61925409715847299,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514563,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087306883203,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)((?:[$^?])|(?i:_|Args|ConsoleFileName|Event|EventArgs|EventSubscriber|ForEach|Input|LastExitCode|Matches|MyInvocation|NestedPromptLevel|Profile|PSBoundParameters|PsCmdlet|PsCulture|PSDebugContext|PSItem|PSCommandPath|PSScriptRoot|PsUICulture|Pwd|Sender|SourceArgs|SourceEventArgs|StackTrace|Switch|This)\\b)((?:\\.(?:\\p{L}|\\d|_)+)*\\b)?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 61925246556635267,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514563,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087306883203,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)(?i:(ConfirmPreference|DebugPreference|ErrorActionPreference|ErrorView|FormatEnumerationLimit|InformationPreference|LogCommandHealthEvent|LogCommandLifecycleEvent|LogEngineHealthEvent|LogEngineLifecycleEvent|LogProviderHealthEvent|LogProviderLifecycleEvent|MaximumAliasCount|MaximumDriveCount|MaximumErrorCount|MaximumFunctionCount|MaximumHistoryCount|MaximumVariableCount|OFS|OutputEncoding|PSCulture|PSDebugContext|PSDefaultParameterValues|PSEmailServer|PSItem|PSModuleAutoLoadingPreference|PSModuleAutoloadingPreference|PSSenderInfo|PSSessionApplicationName|PSSessionConfigurationName|PSSessionOption|ProgressPreference|VerbosePreference|WarningPreference|WhatIfPreference))((?:\\.(?:\\p{L}|\\d|_)+)*\\b)?\\b"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259061530787840,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514563,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 49259087306883203,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:(\\$|@)(global|local|private|script|using|workflow):((?:\\p{L}|\\d|_)+))((?:\\.(?:\\p{L}|\\d|_)+)*\\b)?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087310291075,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514563,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 48414439095795843,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 49259087306883203,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:(\\$)(\\{)(global|local|private|script|using|workflow):([^}]*[^}`])(\\}))((?:\\.(?:\\p{L}|\\d|_)+)*\\b)?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087310291075,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514563,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521962160310,
            b: 36873221949095936,
        },
    ]),(3, vec![
        Scope {
            a: 48414439095795843,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288521962160299,
            b: 36873221949095936,
        },
    ]),(6, vec![
        Scope {
            a: 49259087306883203,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:(\\$|@)((?:\\p{L}|\\d|_)+:)?((?:\\p{L}|\\d|_)+))((?:\\.(?:\\p{L}|\\d|_)+)*\\b)?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087310291075,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514563,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925246539661443,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 49259087306883203,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?i:(\\$)(\\{)((?:\\p{L}|\\d|_)+:)?([^}]*[^}`])(\\}))((?:\\.(?:\\p{L}|\\d|_)+)*\\b)?"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49259087310291075,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514563,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288521962160310,
            b: 36873221949095936,
        },
    ]),(3, vec![
        Scope {
            a: 61925246539661443,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288521962160299,
            b: 36873221949095936,
        },
    ]),(6, vec![
        Scope {
            a: 49259087306883203,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }