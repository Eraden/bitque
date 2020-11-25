
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
      regex: Regex::new("\\b(?:AbstractArray|AbstractChannel|AbstractChar|AbstractDict|AbstractDisplay|AbstractFloat|AbstractIrrational|AbstractMatrix|AbstractRange|AbstractSet|AbstractString|AbstractUnitRange|AbstractVecOrMat|AbstractVector|Any|ArgumentError|Array|AssertionError|BigFloat|BigInt|BitArray|BitMatrix|BitSet|BitVector|Bool|BoundsError|CapturedException|CartesianIndex|CartesianIndices|Cchar|Cdouble|Cfloat|Channel|Char|Cint|Cintmax_t|Clong|Clonglong|Cmd|Colon|Complex|ComplexF16|ComplexF32|ComplexF64|CompositeException|Condition|Cptrdiff_t|Cshort|Csize_t|Cssize_t|Cstring|Cuchar|Cuint|Cuintmax_t|Culong|Culonglong|Cushort|Cvoid|Cwchar_t|Cwstring|DataType|DenseArray|DenseMatrix|DenseVecOrMat|DenseVector|Dict|DimensionMismatch|Dims|DivideError|DomainError|EOFError|Enum|ErrorException|Exception|ExponentialBackOff|Expr|Float16|Float32|Float64|Function|GlobalRef|HTML|IO|IOBuffer|IOContext|IOStream|IdDict|IndexCartesian|IndexLinear|IndexStyle|InexactError|InitError|Int|Int128|Int16|Int32|Int64|Int8|Integer|InterruptException|InvalidStateException|Irrational|KeyError|LinRange|LineNumberNode|LinearIndices|LoadError|MIME|Matrix|Method|MethodError|Missing|MissingException|Module|NTuple|NamedTuple|Nothing|Number|OrdinalRange|OutOfMemoryError|OverflowError|Pair|PartialQuickSort|PermutedDimsArray|Pipe|ProcessFailedException|Ptr|QuoteNode|Rational|RawFD|ReadOnlyMemoryError|Real|ReentrantLock|Ref|Regex|RegexMatch|RoundingMode|SegmentationFault|Set|Signed|Some|StackOverflowError|StepRange|StepRangeLen|StridedArray|StridedMatrix|StridedVecOrMat|StridedVector|String|StringIndexError|SubArray|SubString|SubstitutionString|Symbol|SystemError|Task|Text|TextDisplay|Timer|Tuple|Type|TypeError|TypeVar|UInt|UInt128|UInt16|UInt32|UInt64|UInt8|UndefInitializer|UndefKeywordError|UndefRefError|UndefVarError|Union|UnionAll|UnitRange|Unsigned|Val|Vararg|VecElement|VecOrMat|Vector|VersionNumber|WeakKeyDict|WeakRef)\\b(?=\\{)"),
      scope: vec![
        Scope {
            a: 46453563122581504,
            b: 0,
        },
        Scope {
            a: 61925375352569856,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 8019 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?:AbstractArray|AbstractChannel|AbstractChar|AbstractDict|AbstractDisplay|AbstractFloat|AbstractIrrational|AbstractMatrix|AbstractRange|AbstractSet|AbstractString|AbstractUnitRange|AbstractVecOrMat|AbstractVector|Any|ArgumentError|Array|AssertionError|BigFloat|BigInt|BitArray|BitMatrix|BitSet|BitVector|Bool|BoundsError|CapturedException|CartesianIndex|CartesianIndices|Cchar|Cdouble|Cfloat|Channel|Char|Cint|Cintmax_t|Clong|Clonglong|Cmd|Colon|Complex|ComplexF16|ComplexF32|ComplexF64|CompositeException|Condition|Cptrdiff_t|Cshort|Csize_t|Cssize_t|Cstring|Cuchar|Cuint|Cuintmax_t|Culong|Culonglong|Cushort|Cvoid|Cwchar_t|Cwstring|DataType|DenseArray|DenseMatrix|DenseVecOrMat|DenseVector|Dict|DimensionMismatch|Dims|DivideError|DomainError|EOFError|Enum|ErrorException|Exception|ExponentialBackOff|Expr|Float16|Float32|Float64|Function|GlobalRef|HTML|IO|IOBuffer|IOContext|IOStream|IdDict|IndexCartesian|IndexLinear|IndexStyle|InexactError|InitError|Int|Int128|Int16|Int32|Int64|Int8|Integer|InterruptException|InvalidStateException|Irrational|KeyError|LinRange|LineNumberNode|LinearIndices|LoadError|MIME|Matrix|Method|MethodError|Missing|MissingException|Module|NTuple|NamedTuple|Nothing|Number|OrdinalRange|OutOfMemoryError|OverflowError|Pair|PartialQuickSort|PermutedDimsArray|Pipe|ProcessFailedException|Ptr|QuoteNode|Rational|RawFD|ReadOnlyMemoryError|Real|ReentrantLock|Ref|Regex|RegexMatch|RoundingMode|SegmentationFault|Set|Signed|Some|StackOverflowError|StepRange|StepRangeLen|StridedArray|StridedMatrix|StridedVecOrMat|StridedVector|String|StringIndexError|SubArray|SubString|SubstitutionString|Symbol|SystemError|Task|Text|TextDisplay|Timer|Tuple|Type|TypeError|TypeVar|UInt|UInt128|UInt16|UInt32|UInt64|UInt8|UndefInitializer|UndefKeywordError|UndefRefError|UndefVarError|Union|UnionAll|UnitRange|Unsigned|Val|Vararg|VecElement|VecOrMat|Vector|VersionNumber|WeakKeyDict|WeakRef)\\b"),
      scope: vec![
        Scope {
            a: 61925375352569856,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }