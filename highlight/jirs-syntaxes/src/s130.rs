
#![allow(unused_imports)]
#![allow(dead_code)]

use jirs_syntect::{*, parsing::*};
use lazycell::AtomicLazyCell;

#[inline(always)]
pub fn load() -> SyntaxReference {
        SyntaxReference {  name: "Julia".to_string(),
  file_extensions: vec!["jl".to_string()],
  scope: Scope { a: 844944621174784, b: 0 },
  first_line_match: Some("^#!.*\\bjulia\\s*$".to_string()),
  hidden: false,
  variables: {
    let mut v = std::collections::HashMap::new();
    v.insert("symb_op_ascii".to_string(), "[-+*/\\\\=^:<>~?&$%|!]".to_string());
    v.insert("long_op".to_string(), "(?:\\+=|-=|\\*=|/=|//=|\\\\\\\\=|^=|÷=|%=|<<=|>>=|>>>=|\\|=|&=|:=|=>|$=|\\|\\||&&|<:|>:|\\|>|<\\||//|\\+\\+|<=|>=|->|===|==|!==|!=)".to_string());
    v.insert("symb_id".to_string(), "(?:[^\\s{{symb_lang}}{{symb_op}}0-9](?:[^\\s{{symb_lang}}{{symb_op}}]|!)*)".to_string());
    v.insert("base_funcs".to_string(), "\\b(?:abs|abs2|abspath|accumulate|accumulate!|acos|acosd|acosh|acot|acotd|acoth|acsc|acscd|acsch|adjoint|all|all!|allunique|angle|any|any!|append!|argmax|argmin|ascii|asec|asecd|asech|asin|asind|asinh|asyncmap|asyncmap!|atan|atand|atanh|atexit|atreplinit|axes|backtrace|basename|big|bind|binomial|bitstring|broadcast|broadcast!|bswap|bytes2hex|bytesavailable|cat|catch_backtrace|cbrt|cd|ceil|cglobal|checkbounds|checkindex|chmod|chomp|chop|chown|circcopy!|circshift|circshift!|cis|clamp|clamp!|cld|close|cmp|coalesce|code_lowered|code_typed|codepoint|codeunit|codeunits|collect|complex|conj|conj!|convert|copy|copy!|copysign|copyto!|cos|cosc|cosd|cosh|cospi|cot|cotd|coth|count|count_ones|count_zeros|countlines|cp|csc|cscd|csch|ctime|cumprod|cumprod!|cumsum|cumsum!|current_task|deepcopy|deg2rad|delete!|deleteat!|denominator|detach|devnull|diff|digits|digits!|dirname|disable_sigint|display|displayable|displaysize|div|divrem|download|dropdims|dump|eachcol|eachindex|eachline|eachmatch|eachrow|eachslice|eltype|empty|empty!|endswith|enumerate|eof|eps|error|esc|escape_string|evalfile|exit|exp|exp10|exp2|expanduser|expm1|exponent|extrema|factorial|falses|fd|fdio|fetch|fieldcount|fieldname|fieldnames|fieldoffset|fieldtypes|filemode|filesize|fill|fill!|filter|filter!|finalize|finalizer|findall|findfirst|findlast|findmax|findmax!|findmin|findmin!|findnext|findprev|first|firstindex|fld|fld1|fldmod|fldmod1|flipsign|float|floatmax|floatmin|floor|flush|fma|foldl|foldr|foreach|frexp|fullname|functionloc|gcd|gcdx|gensym|get|get!|get_zero_subnormals|gethostname|getindex|getkey|getpid|getproperty|gperm|hasfield|hash|haskey|hasmethod|hasproperty|hcat|hex2bytes|hex2bytes!|homedir|htol|hton|hvcat|hypot|identity|ifelse|ignorestatus|im|imag|in|include_dependency|include_string|indexin|insert!|instances|intersect|intersect!|inv|invmod|invperm|invpermute!|isabspath|isabstracttype|isapprox|isascii|isassigned|isbits|isbitstype|isblockdev|ischardev|iscntrl|isconcretetype|isconst|isdigit|isdir|isdirpath|isdispatchtuple|isempty|isequal|iseven|isfifo|isfile|isfinite|isimmutable|isinf|isinteger|isinteractive|isless|isletter|islink|islocked|islowercase|ismarked|ismissing|ismount|isnan|isnothing|isnumeric|isodd|isone|isopen|ispath|isperm|ispow2|isprimitivetype|isprint|ispunct|isqrt|isreadable|isreadonly|isready|isreal|issetequal|issetgid|issetuid|issocket|issorted|isspace|issticky|isstructtype|issubnormal|issubset|istaskdone|istaskstarted|istextmime|isuppercase|isvalid|iswritable|isxdigit|iszero|iterate|join|joinpath|keys|keytype|kill|kron|last|lastindex|lcm|ldexp|leading_ones|leading_zeros|length|lock|log|log10|log1p|log2|lowercase|lowercasefirst|lpad|lstat|lstrip|ltoh|macroexpand|map|map!|mapfoldl|mapfoldr|mapreduce|mapslices|mark|match|max|maximum|maximum!|maxintfloat|merge|merge!|methods|min|minimum|minimum!|minmax|missing|mkdir|mkpath|mktemp|mktempdir|mod|mod1|mod2pi|modf|mtime|muladd|mv|nameof|names|ncodeunits|ndigits|ndims|nextfloat|nextind|nextpow|nextprod|normpath|notify|ntoh|ntuple|numerator|objectid|occursin|oftype|one|ones|oneunit|open|operm|pairs|parent|parentindices|parentmodule|parse|partialsort|partialsort!|partialsortperm|partialsortperm!|pathof|permute!|permutedims|permutedims!|pi|pipeline|pointer|pointer_from_objref|pop!|popdisplay|popfirst!|position|powermod|precision|precompile|prepend!|prevfloat|prevind|prevpow|print|println|printstyled|process_exited|process_running|prod|prod!|promote|promote_rule|promote_shape|promote_type|propertynames|push!|pushdisplay|pushfirst!|put!|pwd|rad2deg|rand|randn|range|rationalize|read|read!|readavailable|readbytes!|readchomp|readdir|readline|readlines|readlink|readuntil|real|realpath|redirect_stderr|redirect_stdin|redirect_stdout|redisplay|reduce|reenable_sigint|reim|reinterpret|relpath|rem|rem2pi|repeat|replace|replace!|repr|reset|reshape|resize!|rethrow|retry|reverse|reverse!|reverseind|rm|rot180|rotl90|rotr90|round|rounding|rpad|rsplit|rstrip|run|schedule|searchsorted|searchsortedfirst|searchsortedlast|sec|secd|sech|seek|seekend|seekstart|selectdim|set_zero_subnormals|setdiff|setdiff!|setenv|setindex!|setprecision|setproperty!|setrounding|show|showable|showerror|sign|signbit|signed|significand|similar|sin|sinc|sincos|sind|sinh|sinpi|size|sizehint!|sizeof|skip|skipchars|skipmissing|sleep|something|sort|sort!|sortperm|sortperm!|sortslices|splice!|split|splitdir|splitdrive|splitext|splitpath|sprint|sqrt|stacktrace|startswith|stat|stderr|stdin|stdout|step|stride|strides|string|strip|success|sum|sum!|summary|supertype|symdiff|symdiff!|symlink|systemerror|take!|tan|tand|tanh|task_local_storage|tempdir|tempname|textwidth|thisind|time|time_ns|timedwait|titlecase|to_indices|touch|trailing_ones|trailing_zeros|transcode|transpose|trues|trunc|truncate|trylock|tryparse|typeintersect|typejoin|typemax|typemin|unescape_string|union|union!|unique|unique!|unlock|unmark|unsafe_copyto!|unsafe_load|unsafe_pointer_to_objref|unsafe_read|unsafe_store!|unsafe_string|unsafe_trunc|unsafe_wrap|unsafe_write|unsigned|uperm|uppercase|uppercasefirst|valtype|values|vcat|vec|view|wait|walkdir|which|widemul|widen|withenv|write|xor|yield|yieldto|zero|zeros|zip|applicable|eval|fieldtype|getfield|ifelse|invoke|isa|isdefined|nfields|nothing|setfield!|throw|tuple|typeassert|typeof|undef|include)(?!{{symb_id}})".to_string());
    v.insert("base_macros".to_string(), "\\b(?:__DIR__|__FILE__|__LINE__|__MODULE__|__dot__|allocated|assert|async|boundscheck|cfunction|cmd|debug|deprecate|doc|elapsed|enum|error|eval|evalpoly|fastmath|generated|gensym|goto|inbounds|info|inline|isdefined|label|macroexpand|macroexpand1|noinline|nospecialize|polly|show|simd|specialize|static|sync|task|threadcall|time|timed|timev|view|views|warn)".to_string());
    v.insert("base_module_funcs".to_string(), "\\b(?:Base\\.(?:abs|abs2|abspath|accumulate|accumulate!|acos|acosd|acosh|acot|acotd|acoth|acsc|acscd|acsch|adjoint|all|all!|allunique|angle|any|any!|append!|argmax|argmin|ascii|asec|asecd|asech|asin|asind|asinh|asyncmap|asyncmap!|atan|atand|atanh|atexit|atreplinit|axes|backtrace|basename|big|bind|binomial|bitstring|broadcast|broadcast!|bswap|bytes2hex|bytesavailable|cat|catch_backtrace|cbrt|cd|ceil|cglobal|checkbounds|checkindex|chmod|chomp|chop|chown|circcopy!|circshift|circshift!|cis|clamp|clamp!|cld|close|cmp|coalesce|code_lowered|code_typed|codepoint|codeunit|codeunits|collect|complex|conj|conj!|convert|copy|copy!|copysign|copyto!|cos|cosc|cosd|cosh|cospi|cot|cotd|coth|count|count_ones|count_zeros|countlines|cp|csc|cscd|csch|ctime|cumprod|cumprod!|cumsum|cumsum!|current_task|deepcopy|deg2rad|delete!|deleteat!|denominator|detach|devnull|diff|digits|digits!|dirname|disable_sigint|display|displayable|displaysize|div|divrem|download|dropdims|dump|eachcol|eachindex|eachline|eachmatch|eachrow|eachslice|eltype|empty|empty!|endswith|enumerate|eof|eps|error|esc|escape_string|evalfile|exit|exp|exp10|exp2|expanduser|expm1|exponent|extrema|factorial|falses|fd|fdio|fetch|fieldcount|fieldname|fieldnames|fieldoffset|fieldtypes|filemode|filesize|fill|fill!|filter|filter!|finalize|finalizer|findall|findfirst|findlast|findmax|findmax!|findmin|findmin!|findnext|findprev|first|firstindex|fld|fld1|fldmod|fldmod1|flipsign|float|floatmax|floatmin|floor|flush|fma|foldl|foldr|foreach|frexp|fullname|functionloc|gcd|gcdx|gensym|get|get!|get_zero_subnormals|gethostname|getindex|getkey|getpid|getproperty|gperm|hasfield|hash|haskey|hasmethod|hasproperty|hcat|hex2bytes|hex2bytes!|homedir|htol|hton|hvcat|hypot|identity|ifelse|ignorestatus|im|imag|in|include_dependency|include_string|indexin|insert!|instances|intersect|intersect!|inv|invmod|invperm|invpermute!|isabspath|isabstracttype|isapprox|isascii|isassigned|isbits|isbitstype|isblockdev|ischardev|iscntrl|isconcretetype|isconst|isdigit|isdir|isdirpath|isdispatchtuple|isempty|isequal|iseven|isfifo|isfile|isfinite|isimmutable|isinf|isinteger|isinteractive|isless|isletter|islink|islocked|islowercase|ismarked|ismissing|ismount|isnan|isnothing|isnumeric|isodd|isone|isopen|ispath|isperm|ispow2|isprimitivetype|isprint|ispunct|isqrt|isreadable|isreadonly|isready|isreal|issetequal|issetgid|issetuid|issocket|issorted|isspace|issticky|isstructtype|issubnormal|issubset|istaskdone|istaskstarted|istextmime|isuppercase|isvalid|iswritable|isxdigit|iszero|iterate|join|joinpath|keys|keytype|kill|kron|last|lastindex|lcm|ldexp|leading_ones|leading_zeros|length|lock|log|log10|log1p|log2|lowercase|lowercasefirst|lpad|lstat|lstrip|ltoh|macroexpand|map|map!|mapfoldl|mapfoldr|mapreduce|mapslices|mark|match|max|maximum|maximum!|maxintfloat|merge|merge!|methods|min|minimum|minimum!|minmax|missing|mkdir|mkpath|mktemp|mktempdir|mod|mod1|mod2pi|modf|mtime|muladd|mv|nameof|names|ncodeunits|ndigits|ndims|nextfloat|nextind|nextpow|nextprod|normpath|notify|ntoh|ntuple|numerator|objectid|occursin|oftype|one|ones|oneunit|open|operm|pairs|parent|parentindices|parentmodule|parse|partialsort|partialsort!|partialsortperm|partialsortperm!|pathof|permute!|permutedims|permutedims!|pi|pipeline|pointer|pointer_from_objref|pop!|popdisplay|popfirst!|position|powermod|precision|precompile|prepend!|prevfloat|prevind|prevpow|print|println|printstyled|process_exited|process_running|prod|prod!|promote|promote_rule|promote_shape|promote_type|propertynames|push!|pushdisplay|pushfirst!|put!|pwd|rad2deg|rand|randn|range|rationalize|read|read!|readavailable|readbytes!|readchomp|readdir|readline|readlines|readlink|readuntil|real|realpath|redirect_stderr|redirect_stdin|redirect_stdout|redisplay|reduce|reenable_sigint|reim|reinterpret|relpath|rem|rem2pi|repeat|replace|replace!|repr|reset|reshape|resize!|rethrow|retry|reverse|reverse!|reverseind|rm|rot180|rotl90|rotr90|round|rounding|rpad|rsplit|rstrip|run|schedule|searchsorted|searchsortedfirst|searchsortedlast|sec|secd|sech|seek|seekend|seekstart|selectdim|set_zero_subnormals|setdiff|setdiff!|setenv|setindex!|setprecision|setproperty!|setrounding|show|showable|showerror|sign|signbit|signed|significand|similar|sin|sinc|sincos|sind|sinh|sinpi|size|sizehint!|sizeof|skip|skipchars|skipmissing|sleep|something|sort|sort!|sortperm|sortperm!|sortslices|splice!|split|splitdir|splitdrive|splitext|splitpath|sprint|sqrt|stacktrace|startswith|stat|stderr|stdin|stdout|step|stride|strides|string|strip|success|sum|sum!|summary|supertype|symdiff|symdiff!|symlink|systemerror|take!|tan|tand|tanh|task_local_storage|tempdir|tempname|textwidth|thisind|time|time_ns|timedwait|titlecase|to_indices|touch|trailing_ones|trailing_zeros|transcode|transpose|trues|trunc|truncate|trylock|tryparse|typeintersect|typejoin|typemax|typemin|unescape_string|union|union!|unique|unique!|unlock|unmark|unsafe_copyto!|unsafe_load|unsafe_pointer_to_objref|unsafe_read|unsafe_store!|unsafe_string|unsafe_trunc|unsafe_wrap|unsafe_write|unsigned|uperm|uppercase|uppercasefirst|valtype|values|vcat|vec|view|wait|walkdir|which|widemul|widen|withenv|write|xor|yield|yieldto|zero|zeros|zip)|Broadcast\\.(?:broadcast|broadcast!|broadcast_axes|broadcastable|dotview)|Docs\\.(?:doc)|GC\\.(?:)|Iterators\\.(?:countfrom|cycle|drop|enumerate|flatten|partition|product|repeated|rest|take|zip)|Libc\\.(?:calloc|errno|flush_cstdio|free|gethostname|getpid|malloc|realloc|strerror|strftime|strptime|systemsleep|time|transcode)|MathConstants\\.(?:catalan|e|eulergamma|golden|pi)|Meta\\.(?:isexpr|quot|show_sexpr)|StackTraces\\.(?:stacktrace)|Sys\\.(?:cpu_info|cpu_summary|free_memory|isapple|isbsd|isdragonfly|isexecutable|isfreebsd|isjsvm|islinux|isnetbsd|isopenbsd|isunix|iswindows|loadavg|total_memory|uptime|which)|Threads\\.(?:atomic_add!|atomic_and!|atomic_cas!|atomic_fence|atomic_max!|atomic_min!|atomic_nand!|atomic_or!|atomic_sub!|atomic_xchg!|atomic_xor!|nthreads|threadid)|Core\\.(?:applicable|eval|fieldtype|getfield|ifelse|invoke|isa|isdefined|nfields|nothing|setfield!|throw|tuple|typeassert|typeof|undef))(?!{{symb_id}})".to_string());
    v.insert("base_modules".to_string(), "\\b(?:Base|Broadcast|Docs|GC|Iterators|Libc|MathConstants|Meta|StackTraces|Sys|Threads|Core|Main)\\b".to_string());
    v.insert("base_types".to_string(), "\\b(?:AbstractArray|AbstractChannel|AbstractChar|AbstractDict|AbstractDisplay|AbstractFloat|AbstractIrrational|AbstractMatrix|AbstractRange|AbstractSet|AbstractString|AbstractUnitRange|AbstractVecOrMat|AbstractVector|Any|ArgumentError|Array|AssertionError|BigFloat|BigInt|BitArray|BitMatrix|BitSet|BitVector|Bool|BoundsError|CapturedException|CartesianIndex|CartesianIndices|Cchar|Cdouble|Cfloat|Channel|Char|Cint|Cintmax_t|Clong|Clonglong|Cmd|Colon|Complex|ComplexF16|ComplexF32|ComplexF64|CompositeException|Condition|Cptrdiff_t|Cshort|Csize_t|Cssize_t|Cstring|Cuchar|Cuint|Cuintmax_t|Culong|Culonglong|Cushort|Cvoid|Cwchar_t|Cwstring|DataType|DenseArray|DenseMatrix|DenseVecOrMat|DenseVector|Dict|DimensionMismatch|Dims|DivideError|DomainError|EOFError|Enum|ErrorException|Exception|ExponentialBackOff|Expr|Float16|Float32|Float64|Function|GlobalRef|HTML|IO|IOBuffer|IOContext|IOStream|IdDict|IndexCartesian|IndexLinear|IndexStyle|InexactError|InitError|Int|Int128|Int16|Int32|Int64|Int8|Integer|InterruptException|InvalidStateException|Irrational|KeyError|LinRange|LineNumberNode|LinearIndices|LoadError|MIME|Matrix|Method|MethodError|Missing|MissingException|Module|NTuple|NamedTuple|Nothing|Number|OrdinalRange|OutOfMemoryError|OverflowError|Pair|PartialQuickSort|PermutedDimsArray|Pipe|ProcessFailedException|Ptr|QuoteNode|Rational|RawFD|ReadOnlyMemoryError|Real|ReentrantLock|Ref|Regex|RegexMatch|RoundingMode|SegmentationFault|Set|Signed|Some|StackOverflowError|StepRange|StepRangeLen|StridedArray|StridedMatrix|StridedVecOrMat|StridedVector|String|StringIndexError|SubArray|SubString|SubstitutionString|Symbol|SystemError|Task|Text|TextDisplay|Timer|Tuple|Type|TypeError|TypeVar|UInt|UInt128|UInt16|UInt32|UInt64|UInt8|UndefInitializer|UndefKeywordError|UndefRefError|UndefVarError|Union|UnionAll|UnitRange|Unsigned|Val|Vararg|VecElement|VecOrMat|Vector|VersionNumber|WeakKeyDict|WeakRef)\\b".to_string());
    v.insert("symb_lang".to_string(), "(?:[(){}\\[\\],.;:\'\"`@#])".to_string());
    v.insert("symb_op_unicode".to_string(), "[≤≥¬←→↔↚↛↠↣↦↮⇎⇏⇒⇔⇴⇶⇷⇸⇹⇺⇻⇼⇽⇾⇿⟵⟶⟷⟷⟹⟺⟻⟼⟽⟾⟿⤀⤁⤂⤃⤄⤅⤆⤇⤌⤍⤎⤏⤐⤑⤔⤕⤖⤗⤘⤝⤞⤟⤠⥄⥅⥆⥇⥈⥊⥋⥎⥐⥒⥓⥖⥗⥚⥛⥞⥟⥢⥤⥦⥧⥨⥩⥪⥫⥬⥭⥰⧴⬱⬰⬲⬳⬴⬵⬶⬷⬸⬹⬺⬻⬼⬽⬾⬿⭀⭁⭂⭃⭄⭇⭈⭉⭊⭋⭌￩￫≡≠≢∈∉∋∌⊆⊈⊂⊄⊊∝∊∍∥∦∷∺∻∽∾≁≃≄≅≆≇≈≉≊≋≌≍≎≐≑≒≓≔≕≖≗≘≙≚≛≜≝≞≟≣≦≧≨≩≪≫≬≭≮≯≰≱≲≳≴≵≶≷≸≹≺≻≼≽≾≿⊀⊁⊃⊅⊇⊉⊋⊏⊐⊑⊒⊜⊩⊬⊮⊰⊱⊲⊳⊴⊵⊶⊷⋍⋐⋑⋕⋖⋗⋘⋙⋚⋛⋜⋝⋞⋟⋠⋡⋢⋣⋤⋥⋦⋧⋨⋩⋪⋫⋬⋭⋲⋳⋴⋵⋶⋷⋸⋹⋺⋻⋼⋽⋾⋿⟈⟉⟒⦷⧀⧁⧡⧣⧤⧥⩦⩧⩪⩫⩬⩭⩮⩯⩰⩱⩲⩳⩴⩵⩶⩷⩸⩹⩺⩻⩼⩽⩾⩿⪀⪁⪂⪃⪄⪅⪆⪇⪈⪉⪊⪋⪌⪍⪎⪏⪐⪑⪒⪓⪔⪕⪖⪗⪘⪙⪚⪛⪜⪝⪞⪟⪠⪡⪢⪣⪤⪥⪦⪧⪨⪩⪪⪫⪬⪭⪮⪯⪰⪱⪲⪳⪴⪵⪶⪷⪸⪹⪺⪻⪼⪽⪾⪿⫀⫁⫂⫃⫄⫅⫆⫇⫈⫉⫊⫋⫌⫍⫎⫏⫐⫑⫒⫓⫔⫕⫖⫗⫘⫙⫷⫸⫹⫺⊢⊣⊕⊖⊞⊟∪∨⊔±∓∔∸≂≏⊎⊻⊽⋎⋓⧺⧻⨈⨢⨣⨤⨥⨦⨧⨨⨩⨪⨫⨬⨭⨮⨹⨺⩁⩂⩅⩊⩌⩏⩐⩒⩔⩖⩗⩛⩝⩡⩢⩣÷⋅∘×∩∧⊗⊘⊙⊚⊛⊠⊡⊓∗∙∤⅋≀⊼⋄⋆⋇⋉⋊⋋⋌⋏⋒⟑⦸⦼⦾⦿⧶⧷⨇⨰⨱⨲⨳⨴⨵⨶⨷⨸⨻⨼⨽⩀⩃⩄⩋⩍⩎⩑⩓⩕⩘⩚⩜⩞⩟⩠⫛⊍▷⨝⟕⟖⟗↑↓⇵⟰⟱⤈⤉⤊⤋⤒⤓⥉⥌⥍⥏⥑⥔⥕⥘⥙⥜⥝⥠⥡⥣⥥⥮⥯￪￬]".to_string());
    v.insert("symb_op".to_string(), "(?:{{symb_op_ascii}}|{{symb_op_unicode}})".to_string());
    v
  },
  contexts: {
    let mut v = std::collections::HashMap::new();
    v.insert("#anon_function-propotype-entity_1".to_string(), ContextId { index: 7920 });
    v.insert("constants".to_string(), ContextId { index: 7964 });
    v.insert("#anon_docstrings_0".to_string(), ContextId { index: 7892 });
    v.insert("struct-declarations".to_string(), ContextId { index: 8015 });
    v.insert("#anon_string-quoted-double_2".to_string(), ContextId { index: 7941 });
    v.insert("symbols".to_string(), ContextId { index: 8018 });
    v.insert("#anon_after-expression_0".to_string(), ContextId { index: 7884 });
    v.insert("function-call".to_string(), ContextId { index: 7969 });
    v.insert("function-inline-body".to_string(), ContextId { index: 7975 });
    v.insert("#anon_function-propotype-common_0".to_string(), ContextId { index: 7917 });
    v.insert("#anon_function-parameters-branch_0".to_string(), ContextId { index: 7911 });
    v.insert("#anon_string-quoted-double_3".to_string(), ContextId { index: 7942 });
    v.insert("#anon_brackets_1".to_string(), ContextId { index: 7889 });
    v.insert("string-backtick".to_string(), ContextId { index: 8009 });
    v.insert("#anon_function-call_1".to_string(), ContextId { index: 7903 });
    v.insert("__start".to_string(), ContextId { index: 7957 });
    v.insert("keywords".to_string(), ContextId { index: 7989 });
    v.insert("#anon_function-call-keyword-arguments_3".to_string(), ContextId { index: 7898 });
    v.insert("#anon_function-call-name_0".to_string(), ContextId { index: 7899 });
    v.insert("function-propotype-explict".to_string(), ContextId { index: 7984 });
    v.insert("quote-blocks".to_string(), ContextId { index: 8005 });
    v.insert("#anon_type-parameters-group_0".to_string(), ContextId { index: 7946 });
    v.insert("function-inline-declaration".to_string(), ContextId { index: 7976 });
    v.insert("lambda-function-branch".to_string(), ContextId { index: 7992 });
    v.insert("#anon_string-backtick_1".to_string(), ContextId { index: 7933 });
    v.insert("macro-body".to_string(), ContextId { index: 7994 });
    v.insert("maybe-identifiers".to_string(), ContextId { index: 7999 });
    v.insert("possible-types".to_string(), ContextId { index: 8003 });
    v.insert("function-like-object-method-explict".to_string(), ContextId { index: 7977 });
    v.insert("#anon_docstrings_1".to_string(), ContextId { index: 7893 });
    v.insert("#anon_where-clause-three-terms_1".to_string(), ContextId { index: 7948 });
    v.insert("supported-types".to_string(), ContextId { index: 8017 });
    v.insert("#anon_where-clause-three-terms_0".to_string(), ContextId { index: 7947 });
    v.insert("__main".to_string(), ContextId { index: 7956 });
    v.insert("function-call-name".to_string(), ContextId { index: 7972 });
    v.insert("#anon_interpolated-julia_0".to_string(), ContextId { index: 7923 });
    v.insert("#anon_string-quoted-double_4".to_string(), ContextId { index: 7943 });
    v.insert("#anon_where-clause-three-terms_4".to_string(), ContextId { index: 7951 });
    v.insert("#anon_string-quoted-double-block_0".to_string(), ContextId { index: 7934 });
    v.insert("#anon_function-call_3".to_string(), ContextId { index: 7905 });
    v.insert("#anon_string-quoted-single_0".to_string(), ContextId { index: 7944 });
    v.insert("block-body".to_string(), ContextId { index: 7959 });
    v.insert("main".to_string(), ContextId { index: 7998 });
    v.insert("#anon_lambda-function-single-branch_1".to_string(), ContextId { index: 7927 });
    v.insert("#anon_lambda-function-arguments_0".to_string(), ContextId { index: 7925 });
    v.insert("lambda-function-single-branch".to_string(), ContextId { index: 7993 });
    v.insert("#anon_interpolated-julia_1".to_string(), ContextId { index: 7924 });
    v.insert("#anon_function-call-name_1".to_string(), ContextId { index: 7900 });
    v.insert("macro-declarations".to_string(), ContextId { index: 7995 });
    v.insert("#anon_function-call-name_2".to_string(), ContextId { index: 7901 });
    v.insert("#anon_where-clause-three-terms_2".to_string(), ContextId { index: 7949 });
    v.insert("strings".to_string(), ContextId { index: 8013 });
    v.insert("operators".to_string(), ContextId { index: 8001 });
    v.insert("struct-body".to_string(), ContextId { index: 8014 });
    v.insert("#anon_quote-include_0".to_string(), ContextId { index: 7930 });
    v.insert("function-parameters-branch".to_string(), ContextId { index: 7980 });
    v.insert("#anon_after-expression_2".to_string(), ContextId { index: 7886 });
    v.insert("#anon_quote-include_1".to_string(), ContextId { index: 7931 });
    v.insert("#anon_where-clause_0".to_string(), ContextId { index: 7954 });
    v.insert("function-parameters-with-default-branch".to_string(), ContextId { index: 7981 });
    v.insert("quote-include".to_string(), ContextId { index: 8007 });
    v.insert("quote-body".to_string(), ContextId { index: 8006 });
    v.insert("#anon_where-clause-three-terms_3".to_string(), ContextId { index: 7950 });
    v.insert("#anon_string-quoted-double_0".to_string(), ContextId { index: 7939 });
    v.insert("quotes".to_string(), ContextId { index: 8008 });
    v.insert("codesection".to_string(), ContextId { index: 7962 });
    v.insert("function-parameters".to_string(), ContextId { index: 7979 });
    v.insert("#anon_function-parameters-branch_1".to_string(), ContextId { index: 7912 });
    v.insert("macro-propotype".to_string(), ContextId { index: 7996 });
    v.insert("#anon_comments_1".to_string(), ContextId { index: 7891 });
    v.insert("where-clause".to_string(), ContextId { index: 8020 });
    v.insert("blocks".to_string(), ContextId { index: 7960 });
    v.insert("#anon_function-parameters-with-default-branch_2".to_string(), ContextId { index: 7915 });
    v.insert("#anon_function-propotype-common_1".to_string(), ContextId { index: 7918 });
    v.insert("function-propotype-parameters-group".to_string(), ContextId { index: 7986 });
    v.insert("interpolated-julia".to_string(), ContextId { index: 7988 });
    v.insert("string-quoted-single".to_string(), ContextId { index: 8012 });
    v.insert("function-propotype-inline".to_string(), ContextId { index: 7985 });
    v.insert("function-call-keyword-arguments".to_string(), ContextId { index: 7971 });
    v.insert("#anon_function-like-object-method-explict_0".to_string(), ContextId { index: 7907 });
    v.insert("lambda-function-arguments".to_string(), ContextId { index: 7990 });
    v.insert("#anon_function-call-keyword-arguments_2".to_string(), ContextId { index: 7897 });
    v.insert("#anon_function-call_2".to_string(), ContextId { index: 7904 });
    v.insert("#anon_function-call_4".to_string(), ContextId { index: 7906 });
    v.insert("#anon_function-like-object-method-inline_0".to_string(), ContextId { index: 7909 });
    v.insert("escaped-char".to_string(), ContextId { index: 7966 });
    v.insert("function-call-or-inline-declaration".to_string(), ContextId { index: 7973 });
    v.insert("#anon_macro-propotype_0".to_string(), ContextId { index: 7928 });
    v.insert("#anon_after-expression_1".to_string(), ContextId { index: 7885 });
    v.insert("#anon_string-quoted-double-block_3".to_string(), ContextId { index: 7937 });
    v.insert("#anon_where-clause_1".to_string(), ContextId { index: 7955 });
    v.insert("function-propotype-common".to_string(), ContextId { index: 7982 });
    v.insert("supported-modules".to_string(), ContextId { index: 8016 });
    v.insert("quote-block-body".to_string(), ContextId { index: 8004 });
    v.insert("#anon_where-clause-two-terms_0".to_string(), ContextId { index: 7953 });
    v.insert("parametric-types".to_string(), ContextId { index: 8002 });
    v.insert("#anon_brackets_0".to_string(), ContextId { index: 7888 });
    v.insert("#anon_function-call-keyword-arguments_1".to_string(), ContextId { index: 7896 });
    v.insert("lambda-function-body".to_string(), ContextId { index: 7991 });
    v.insert("#anon_function-call-keyword-arguments_0".to_string(), ContextId { index: 7895 });
    v.insert("type-parameters-group".to_string(), ContextId { index: 8019 });
    v.insert("#anon_function-like-object-method-explict_1".to_string(), ContextId { index: 7908 });
    v.insert("not-lambda-function".to_string(), ContextId { index: 8000 });
    v.insert("#anon_function-propotype-parameters-group_0".to_string(), ContextId { index: 7922 });
    v.insert("#anon_possible-types_0".to_string(), ContextId { index: 7929 });
    v.insert("#anon_symbols_0".to_string(), ContextId { index: 7945 });
    v.insert("function-like-object-method-inline".to_string(), ContextId { index: 7978 });
    v.insert("macros".to_string(), ContextId { index: 7997 });
    v.insert("#anon_function-call-arguments-group_0".to_string(), ContextId { index: 7894 });
    v.insert("#anon_function-like-object-method-inline_1".to_string(), ContextId { index: 7910 });
    v.insert("#anon_function-parameters-with-default-branch_0".to_string(), ContextId { index: 7913 });
    v.insert("#anon_string-quoted-double_1".to_string(), ContextId { index: 7940 });
    v.insert("#anon_where-clause-three-terms_5".to_string(), ContextId { index: 7952 });
    v.insert("#anon_string-quoted-double-block_4".to_string(), ContextId { index: 7938 });
    v.insert("function-propotype-entity".to_string(), ContextId { index: 7983 });
    v.insert("where-clause-three-terms".to_string(), ContextId { index: 8021 });
    v.insert("#anon_after-expression_3".to_string(), ContextId { index: 7887 });
    v.insert("docstrings".to_string(), ContextId { index: 7965 });
    v.insert("#anon_string-quoted-double-block_2".to_string(), ContextId { index: 7936 });
    v.insert("function-call-arguments-group".to_string(), ContextId { index: 7970 });
    v.insert("#anon_function-propotype-explict_0".to_string(), ContextId { index: 7921 });
    v.insert("function-body".to_string(), ContextId { index: 7968 });
    v.insert("function-declarations".to_string(), ContextId { index: 7974 });
    v.insert("brackets".to_string(), ContextId { index: 7961 });
    v.insert("#anon_comments_0".to_string(), ContextId { index: 7890 });
    v.insert("#anon_function-parameters-with-default-branch_3".to_string(), ContextId { index: 7916 });
    v.insert("#anon_function-call_0".to_string(), ContextId { index: 7902 });
    v.insert("#anon_lambda-function-single-branch_0".to_string(), ContextId { index: 7926 });
    v.insert("identifiers".to_string(), ContextId { index: 7987 });
    v.insert("#anon_string-backtick_0".to_string(), ContextId { index: 7932 });
    v.insert("after-expression".to_string(), ContextId { index: 7958 });
    v.insert("comments".to_string(), ContextId { index: 7963 });
    v.insert("string-quoted-double".to_string(), ContextId { index: 8010 });
    v.insert("#anon_function-propotype-entity_0".to_string(), ContextId { index: 7919 });
    v.insert("string-quoted-double-block".to_string(), ContextId { index: 8011 });
    v.insert("where-clause-two-terms".to_string(), ContextId { index: 8022 });
    v.insert("#anon_function-parameters-with-default-branch_1".to_string(), ContextId { index: 7914 });
    v.insert("#anon_string-quoted-double-block_1".to_string(), ContextId { index: 7935 });
    v.insert("escaped-unicode-char".to_string(), ContextId { index: 7967 });
    v
  }
} }