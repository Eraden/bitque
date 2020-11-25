
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
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(arguments)\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 49259061550448791,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(Array|ArrayBuffer|Atomics|Boolean|DataView|Date|Float32Array|Float64Array|Function|Generator\n|GeneratorFunction|Int8Array|Int16Array|Int32Array|Intl|Map|Number|Object|Proxy\n|Reflect|RegExp|Set|SharedArrayBuffer|SIMD|String|Symbol|TypedArray\n|Uint8Array|Uint16Array|Uint32Array|Uint8ClampedArray|WeakMap|WeakSet)\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 61925366759751831,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))((Eval|Internal|Range|Reference|Syntax|Type|URI)?Error)\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 61925366772334743,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(Promise)\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 61925366921953431,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(clear(Interval|Timeout)|decodeURI|decodeURIComponent|encodeURI|encodeURIComponent|escape|eval|\nisFinite|isNaN|parseFloat|parseInt|require|set(Interval|Timeout)|super|unescape|uneval)(?=\\s*\\()"),
      scope: vec![
        Scope {
            a: 61925255095451648,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(Math)(?:\\s*(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))\\s*(?:\n(abs|acos|acosh|asin|asinh|atan|atan2|atanh|cbrt|ceil|clz32|cos|cosh|exp|\nexpm1|floor|fround|hypot|imul|log|log10|log1p|log2|max|min|pow|random|\nround|sign|sin|sinh|sqrt|tan|tanh|trunc)\n|\n(E|LN10|LN2|LOG10E|LOG2E|PI|SQRT1_2|SQRT2)))?\\b(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925409761263767,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925255142441111,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 61925409719649124,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(console)(?:\\s*(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))\\s*(\nassert|clear|count|debug|dir|error|group|groupCollapsed|groupEnd|info|log\n|profile|profileEnd|table|time|timeEnd|timeStamp|trace|warn))?\\b(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925366808510615,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925255139360919,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(JSON)(?:\\s*(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))\\s*(parse|stringify))?\\b(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925409706868887,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925255088046231,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(import)\\s*(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))\\s*(meta)\\b(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636636717449367,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925246510893561,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(new)\\s*(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))\\s*(target)\\b(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 52636628113752215,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925246510891407,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) (?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}]))) \\s* (?:\n(constructor|length|prototype|__proto__)\n|\n(EPSILON|MAX_SAFE_INTEGER|MAX_VALUE|MIN_SAFE_INTEGER|MIN_VALUE|NEGATIVE_INFINITY|POSITIVE_INFINITY))\\b(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925246510891159,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925409714274304,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) (?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.)) \\b (?:\n(document|event|navigator|performance|screen|window)\n|\n(AnalyserNode|ArrayBufferView|Attr|AudioBuffer|AudioBufferSourceNode|AudioContext|AudioDestinationNode|AudioListener\n|AudioNode|AudioParam|BatteryManager|BeforeUnloadEvent|BiquadFilterNode|Blob|BufferSource|ByteString|CSS|CSSConditionRule\n|CSSCounterStyleRule|CSSGroupingRule|CSSMatrix|CSSMediaRule|CSSPageRule|CSSPrimitiveValue|CSSRule|CSSRuleList|CSSStyleDeclaration\n|CSSStyleRule|CSSStyleSheet|CSSSupportsRule|CSSValue|CSSValueList|CanvasGradient|CanvasImageSource|CanvasPattern\n|CanvasRenderingContext2D|ChannelMergerNode|ChannelSplitterNode|CharacterData|ChromeWorker|CloseEvent|Comment|CompositionEvent\n|Console|ConvolverNode|Coordinates|Credential|CredentialsContainer|Crypto|CryptoKey|CustomEvent|DOMError|DOMException\n|DOMHighResTimeStamp|DOMImplementation|DOMString|DOMStringList|DOMStringMap|DOMTimeStamp|DOMTokenList|DataTransfer\n|DataTransferItem|DataTransferItemList|DedicatedWorkerGlobalScope|DelayNode|DeviceProximityEvent|DirectoryEntry\n|DirectoryEntrySync|DirectoryReader|DirectoryReaderSync|Document|DocumentFragment|DocumentTouch|DocumentType|DragEvent\n|DynamicsCompressorNode|Element|Entry|EntrySync|ErrorEvent|Event|EventListener|EventSource|EventTarget|FederatedCredential\n|FetchEvent|File|FileEntry|FileEntrySync|FileException|FileList|FileReader|FileReaderSync|FileSystem|FileSystemSync\n|FontFace|FormData|GainNode|Gamepad|GamepadButton|GamepadEvent|Geolocation|GlobalEventHandlers|HTMLAnchorElement\n|HTMLAreaElement|HTMLAudioElement|HTMLBRElement|HTMLBaseElement|HTMLBodyElement|HTMLButtonElement|HTMLCanvasElement\n|HTMLCollection|HTMLContentElement|HTMLDListElement|HTMLDataElement|HTMLDataListElement|HTMLDialogElement|HTMLDivElement\n|HTMLDocument|HTMLElement|HTMLEmbedElement|HTMLFieldSetElement|HTMLFontElement|HTMLFormControlsCollection|HTMLFormElement\n|HTMLHRElement|HTMLHeadElement|HTMLHeadingElement|HTMLHtmlElement|HTMLIFrameElement|HTMLImageElement|HTMLInputElement\n|HTMLKeygenElement|HTMLLIElement|HTMLLabelElement|HTMLLegendElement|HTMLLinkElement|HTMLMapElement|HTMLMediaElement\n|HTMLMetaElement|HTMLMeterElement|HTMLModElement|HTMLOListElement|HTMLObjectElement|HTMLOptGroupElement|HTMLOptionElement\n|HTMLOptionsCollection|HTMLOutputElement|HTMLParagraphElement|HTMLParamElement|HTMLPreElement|HTMLProgressElement\n|HTMLQuoteElement|HTMLScriptElement|HTMLSelectElement|HTMLShadowElement|HTMLSourceElement|HTMLSpanElement|HTMLStyleElement\n|HTMLTableCaptionElement|HTMLTableCellElement|HTMLTableColElement|HTMLTableDataCellElement|HTMLTableElement|HTMLTableHeaderCellElement\n|HTMLTableRowElement|HTMLTableSectionElement|HTMLTextAreaElement|HTMLTimeElement|HTMLTitleElement|HTMLTrackElement\n|HTMLUListElement|HTMLUnknownElement|HTMLVideoElement|HashChangeEvent|History|IDBCursor|IDBCursorWithValue|IDBDatabase\n|IDBEnvironment|IDBFactory|IDBIndex|IDBKeyRange|IDBMutableFile|IDBObjectStore|IDBOpenDBRequest|IDBRequest|IDBTransaction\n|IDBVersionChangeEvent|IIRFilterNode|IdentityManager|ImageBitmap|ImageBitmapFactories|ImageData|Index|InputDeviceCapabilities\n|InputEvent|InstallEvent|InstallTrigger|KeyboardEvent|LinkStyle|LocalFileSystem|LocalFileSystemSync|Location|MIDIAccess\n|MIDIConnectionEvent|MIDIInput|MIDIInputMap|MIDIOutputMap|MediaElementAudioSourceNode|MediaError|MediaKeyMessageEvent\n|MediaKeySession|MediaKeyStatusMap|MediaKeySystemAccess|MediaKeySystemConfiguration|MediaKeys|MediaRecorder|MediaStream\n|MediaStreamAudioDestinationNode|MediaStreamAudioSourceNode|MessageChannel|MessageEvent|MessagePort|MouseEvent\n|MutationObserver|MutationRecord|NamedNodeMap|Navigator|NavigatorConcurrentHardware|NavigatorGeolocation|NavigatorID\n|NavigatorLanguage|NavigatorOnLine|Node|NodeFilter|NodeIterator|NodeList|NonDocumentTypeChildNode|Notification\n|OfflineAudioCompletionEvent|OfflineAudioContext|OscillatorNode|PageTransitionEvent|PannerNode|ParentNode|PasswordCredential\n|Path2D|PaymentAddress|PaymentRequest|PaymentResponse|Performance|PerformanceEntry|PerformanceFrameTiming|PerformanceMark\n|PerformanceMeasure|PerformanceNavigation|PerformanceNavigationTiming|PerformanceObserver|PerformanceObserverEntryList\n|PerformanceResourceTiming|PerformanceTiming|PeriodicSyncEvent|PeriodicWave|Plugin|Point|PointerEvent|PopStateEvent\n|PortCollection|Position|PositionError|PositionOptions|PresentationConnectionClosedEvent|PresentationConnectionList\n|PresentationReceiver|ProcessingInstruction|ProgressEvent|PromiseRejectionEvent|PushEvent|PushRegistrationManager\n|RTCCertificate|RTCConfiguration|RTCPeerConnection|RTCSessionDescriptionCallback|RTCStatsReport|RadioNodeList|RandomSource\n|Range|ReadableByteStream|RenderingContext|SVGAElement|SVGAngle|SVGAnimateColorElement|SVGAnimateElement|SVGAnimateMotionElement\n|SVGAnimateTransformElement|SVGAnimatedAngle|SVGAnimatedBoolean|SVGAnimatedEnumeration|SVGAnimatedInteger|SVGAnimatedLength\n|SVGAnimatedLengthList|SVGAnimatedNumber|SVGAnimatedNumberList|SVGAnimatedPoints|SVGAnimatedPreserveAspectRatio\n|SVGAnimatedRect|SVGAnimatedString|SVGAnimatedTransformList|SVGAnimationElement|SVGCircleElement|SVGClipPathElement\n|SVGCursorElement|SVGDefsElement|SVGDescElement|SVGElement|SVGEllipseElement|SVGEvent|SVGFilterElement|SVGFontElement\n|SVGFontFaceElement|SVGFontFaceFormatElement|SVGFontFaceNameElement|SVGFontFaceSrcElement|SVGFontFaceUriElement\n|SVGForeignObjectElement|SVGGElement|SVGGlyphElement|SVGGradientElement|SVGHKernElement|SVGImageElement|SVGLength\n|SVGLengthList|SVGLineElement|SVGLinearGradientElement|SVGMPathElement|SVGMaskElement|SVGMatrix|SVGMissingGlyphElement\n|SVGNumber|SVGNumberList|SVGPathElement|SVGPatternElement|SVGPoint|SVGPolygonElement|SVGPolylineElement|SVGPreserveAspectRatio\n|SVGRadialGradientElement|SVGRect|SVGRectElement|SVGSVGElement|SVGScriptElement|SVGSetElement|SVGStopElement|SVGStringList\n|SVGStylable|SVGStyleElement|SVGSwitchElement|SVGSymbolElement|SVGTRefElement|SVGTSpanElement|SVGTests|SVGTextElement\n|SVGTextPositioningElement|SVGTitleElement|SVGTransform|SVGTransformList|SVGTransformable|SVGUseElement|SVGVKernElement\n|SVGViewElement|ServiceWorker|ServiceWorkerContainer|ServiceWorkerGlobalScope|ServiceWorkerRegistration|ServiceWorkerState\n|ShadowRoot|SharedWorker|SharedWorkerGlobalScope|SourceBufferList|StereoPannerNode|Storage|StorageEvent|StyleSheet\n|StyleSheetList|SubtleCrypto|SyncEvent|Text|TextMetrics|TimeEvent|TimeRanges|Touch|TouchEvent|TouchList|Transferable\n|TreeWalker|UIEvent|USVString|VRDisplayCapabilities|ValidityState|WaveShaperNode|WebGL|WebGLActiveInfo|WebGLBuffer\n|WebGLContextEvent|WebGLFramebuffer|WebGLProgram|WebGLRenderbuffer|WebGLRenderingContext|WebGLShader|WebGLShaderPrecisionFormat\n|WebGLTexture|WebGLTimerQueryEXT|WebGLTransformFeedback|WebGLUniformLocation|WebGLVertexArrayObject|WebGLVertexArrayObjectOES\n|WebSocket|WebSockets|WebVTT|WheelEvent|Window|WindowBase64|WindowEventHandlers|WindowTimers|Worker|WorkerGlobalScope\n|WorkerLocation|WorkerNavigator|XMLHttpRequest|XMLHttpRequestEventTarget|XMLSerializer|XPathExpression|XPathResult\n|XSLTProcessor))\\b(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925246550212759,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925366809297047,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) (?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}]))) \\s* (?:\n(ATTRIBUTE_NODE|CDATA_SECTION_NODE|COMMENT_NODE|DOCUMENT_FRAGMENT_NODE|DOCUMENT_NODE|DOCUMENT_TYPE_NODE\n|DOMSTRING_SIZE_ERR|ELEMENT_NODE|ENTITY_NODE|ENTITY_REFERENCE_NODE|HIERARCHY_REQUEST_ERR|INDEX_SIZE_ERR\n|INUSE_ATTRIBUTE_ERR|INVALID_CHARACTER_ERR|NO_DATA_ALLOWED_ERR|NO_MODIFICATION_ALLOWED_ERR|NOT_FOUND_ERR\n|NOT_SUPPORTED_ERR|NOTATION_NODE|PROCESSING_INSTRUCTION_NODE|TEXT_NODE|WRONG_DOCUMENT_ERR)\n|\n(_content|[xyz]|abbr|above|accept|acceptCharset|accessKey|action|align|[av]Link(?:color)?|all|alt|anchors|appCodeName\n|appCore|applets|appMinorVersion|appName|appVersion|archive|areas|arguments|attributes|availHeight|availLeft|availTop\n|availWidth|axis|background|backgroundColor|backgroundImage|below|bgColor|body|border|borderBottomWidth|borderColor\n|borderLeftWidth|borderRightWidth|borderStyle|borderTopWidth|borderWidth|bottom|bufferDepth|callee|caller|caption\n|cellPadding|cells|cellSpacing|ch|characterSet|charset|checked|childNodes|chOff|cite|classes|className|clear\n|clientInformation|clip|clipBoardData|closed|code|codeBase|codeType|color|colorDepth|cols|colSpan|compact|complete\n|components|content|controllers|cookie|cookieEnabled|cords|cpuClass|crypto|current|data|dateTime|declare|defaultCharset\n|defaultChecked|defaultSelected|defaultStatus|defaultValue|defaultView|defer|description|dialogArguments|dialogHeight\n|dialogLeft|dialogTop|dialogWidth|dir|directories|disabled|display|docmain|doctype|documentElement|elements|embeds\n|enabledPlugin|encoding|enctype|entities|event|expando|external|face|fgColor|filename|firstChild|fontFamily|fontSize\n|fontWeight|form|formName|forms|frame|frameBorder|frameElement|frames|hasFocus|hash|headers|height|history|host\n|hostname|href|hreflang|hspace|htmlFor|httpEquiv|id|ids|ignoreCase|images|implementation|index|innerHeight|innerWidth\n|input|isMap|label|lang|language|lastChild|lastIndex|lastMatch|lastModified|lastParen|layer[sXY]|left|leftContext\n|lineHeight|link|linkColor|links|listStyleType|localName|location|locationbar|longDesc|lowsrc|lowSrc|marginBottom\n|marginHeight|marginLeft|marginRight|marginTop|marginWidth|maxLength|media|menubar|method|mimeTypes|multiline|multiple\n|name|nameProp|namespaces|namespaceURI|next|nextSibling|nodeName|nodeType|nodeValue|noHref|noResize|noShade|notationName\n|notations|noWrap|object|offscreenBuffering|onLine|onreadystatechange|opener|opsProfile|options|oscpu|outerHeight\n|outerWidth|ownerDocument|paddingBottom|paddingLeft|paddingRight|paddingTop|page[XY]|page[XY]Offset|parent|parentLayer\n|parentNode|parentWindow|pathname|personalbar|pixelDepth|pkcs11|platform|plugins|port|prefix|previous|previousDibling\n|product|productSub|profile|profileend|prompt|prompter|protocol|publicId|readOnly|readyState|referrer|rel|responseText\n|responseXML|rev|right|rightContext|rowIndex|rows|rowSpan|rules|scheme|scope|screen[XY]|screenLeft|screenTop|scripts\n|scrollbars|scrolling|sectionRowIndex|security|securityPolicy|selected|selectedIndex|selection|self|shape|siblingAbove\n|siblingBelow|size|source|specified|standby|start|status|statusbar|statusText|style|styleSheets|suffixes|summary\n|systemId|systemLanguage|tagName|tags|target|tBodies|text|textAlign|textDecoration|textIndent|textTransform|tFoot|tHead\n|title|toolbar|top|type|undefined|uniqueID|updateInterval|URL|URLUnencoded|useMap|userAgent|userLanguage|userProfile\n|vAlign|value|valueType|vendor|vendorSub|version|visibility|vspace|whiteSpace|width|X[MS]LDocument|zIndex))\\b(?!\\$|\\s*(<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<]|\\<\\s*([_$\\p{L}]|(\\{([^\\{\\}]|(\\{[^\\{\\}]*\\}))*\\})|(\\(([^\\(\\)]|(\\([^\\(\\)]*\\)))*\\))|(\\[([^\\[\\]]|(\\[[^\\[\\]]*\\]))*\\]))([^=<>]|=[^<])*\\>)*>\\s*)?\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925409758970007,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925246510891841,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(Buffer|EventEmitter|Server|Pipe|Socket|REPLServer|ReadStream|WriteStream|Stream\n|Inflate|Deflate|InflateRaw|DeflateRaw|GZip|GUnzip|Unzip|Zip)\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 61925366801498263,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x)(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(process)(?:(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))(?:\n  (arch|argv|config|connected|env|execArgv|execPath|exitCode|mainModule|pid|platform|release|stderr|stdin|stdout|title|version|versions)\n  |\n  (abort|chdir|cwd|disconnect|exit|[sg]ete?[gu]id|send|[sg]etgroups|initgroups|kill|memoryUsage|nextTick|umask|uptime|hrtime)\n))?\\b(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925246550017441,
            b: 42502721483309056,
        },
    ]),(2, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925246510892449,
            b: 42502721483309056,
        },
    ]),(5, vec![
        Scope {
            a: 61925255179993239,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(?:(exports)|(module)(?:(?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}])))(exports|id|filename|loaded|parent|children))?)\\b(?!\\$)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 61925375399035643,
            b: 42502721483309056,
        },
    ]),(2, vec![
        Scope {
            a: 61925375399035643,
            b: 42502721483309056,
        },
    ]),(3, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 61925375399035643,
            b: 42502721483309056,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?<![_$\\p{L}\\p{N}])(?:(?<=\\.\\.\\.)|(?<!\\.))(global|GLOBAL|root|__dirname|__filename)\\b(?!\\$)"),
      scope: vec![
        Scope {
            a: 61925246550016714,
            b: 42502721483309056,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(?x) (?:(\\.)|(\\?\\.(?!\\s*[\\p{Nd}]))) \\s*\n(?:\n (on(?:Rowsinserted|Rowsdelete|Rowenter|Rowexit|Resize|Resizestart|Resizeend|Reset|\n   Readystatechange|Mouseout|Mouseover|Mousedown|Mouseup|Mousemove|\n   Before(?:cut|deactivate|unload|update|paste|print|editfocus|activate)|\n   Blur|Scrolltop|Submit|Select|Selectstart|Selectionchange|Hover|Help|\n   Change|Contextmenu|Controlselect|Cut|Cellchange|Clock|Close|Deactivate|\n   Datasetchanged|Datasetcomplete|Dataavailable|Drop|Drag|Dragstart|Dragover|\n   Dragdrop|Dragenter|Dragend|Dragleave|Dblclick|Unload|Paste|Propertychange|Error|\n   Errorupdate|Keydown|Keyup|Keypress|Focus|Load|Activate|Afterupdate|Afterprint|Abort)\n ) |\n (shift|showModelessDialog|showModalDialog|showHelp|scroll|scrollX|scrollByPages|\n   scrollByLines|scrollY|scrollTo|stop|strike|sizeToContent|sidebar|signText|sort|\n   sup|sub|substr|substring|splice|split|send|set(?:Milliseconds|Seconds|Minutes|Hours|\n   Month|Year|FullYear|Date|UTC(?:Milliseconds|Seconds|Minutes|Hours|Month|FullYear|Date)|\n   Time|Hotkeys|Cursor|ZOptions|Active|Resizable|RequestHeader)|search|slice|\n   savePreferences|small|home|handleEvent|navigate|char|charCodeAt|charAt|concat|\n   contextual|confirm|compile|clear|captureEvents|call|createStyleSheet|createPopup|\n   createEventObject|to(?:GMTString|UTCString|String|Source|UpperCase|LowerCase|LocaleString)|\n   test|taint|taintEnabled|indexOf|italics|disableExternalCapture|dump|detachEvent|unshift|\n   untaint|unwatch|updateCommands|join|javaEnabled|pop|push|plugins.refresh|paddings|parse|\n   print|prompt|preference|enableExternalCapture|exec|execScript|valueOf|UTC|find|file|\n   fileModifiedDate|fileSize|fileCreatedDate|fileUpdatedDate|fixed|fontsize|fontcolor|\n   forward|fromCharCode|watch|link|load|lastIndexOf|anchor|attachEvent|atob|apply|alert|\n   abort|routeEvents|resize|resizeBy|resizeTo|recalc|returnValue|replace|reverse|reload|\n   releaseCapture|releaseEvents|go|get(?:Milliseconds|Seconds|Minutes|Hours|Month|Day|Year|FullYear|\n   Time|Date|TimezoneOffset|UTC(?:Milliseconds|Seconds|Minutes|Hours|Day|Month|FullYear|Date)|\n   Attention|Selection|ResponseHeader|AllResponseHeaders)|moveBy|moveBelow|moveTo|\n   moveToAbsolute|moveAbove|mergeAttributes|match|margins|btoa|big|bold|borderWidths|blink|back\n ) |\n (acceptNode|add|addEventListener|addTextTrack|adoptNode|after|animate|append|\n   appendChild|appendData|before|blur|canPlayType|captureStream|\n   caretPositionFromPoint|caretRangeFromPoint|checkValidity|clear|click|\n   cloneContents|cloneNode|cloneRange|close|closest|collapse|\n   compareBoundaryPoints|compareDocumentPosition|comparePoint|contains|\n   convertPointFromNode|convertQuadFromNode|convertRectFromNode|createAttribute|\n   createAttributeNS|createCaption|createCDATASection|createComment|\n   createContextualFragment|createDocument|createDocumentFragment|\n   createDocumentType|createElement|createElementNS|createEntityReference|\n   createEvent|createExpression|createHTMLDocument|createNodeIterator|\n   createNSResolver|createProcessingInstruction|createRange|createShadowRoot|\n   createTBody|createTextNode|createTFoot|createTHead|createTreeWalker|delete|\n   deleteCaption|deleteCell|deleteContents|deleteData|deleteRow|deleteTFoot|\n   deleteTHead|detach|disconnect|dispatchEvent|elementFromPoint|elementsFromPoint|\n   enableStyleSheetsForSet|entries|evaluate|execCommand|exitFullscreen|\n   exitPointerLock|expand|extractContents|fastSeek|firstChild|focus|forEach|get|\n   getAll|getAnimations|getAttribute|getAttributeNames|getAttributeNode|\n   getAttributeNodeNS|getAttributeNS|getBoundingClientRect|getBoxQuads|\n   getClientRects|getContext|getDestinationInsertionPoints|getElementById|\n   getElementsByClassName|getElementsByName|getElementsByTagName|\n   getElementsByTagNameNS|getItem|getNamedItem|getSelection|getStartDate|\n   getVideoPlaybackQuality|has|hasAttribute|hasAttributeNS|hasAttributes|\n   hasChildNodes|hasFeature|hasFocus|importNode|initEvent|insertAdjacentElement|\n   insertAdjacentHTML|insertAdjacentText|insertBefore|insertCell|insertData|\n   insertNode|insertRow|intersectsNode|isDefaultNamespace|isEqualNode|\n   isPointInRange|isSameNode|item|key|keys|lastChild|load|lookupNamespaceURI|\n   lookupPrefix|matches|move|moveAttribute|moveAttributeNode|moveChild|\n   moveNamedItem|namedItem|nextNode|nextSibling|normalize|observe|open|\n   parentNode|pause|play|postMessage|prepend|preventDefault|previousNode|\n   previousSibling|probablySupportsContext|queryCommandEnabled|\n   queryCommandIndeterm|queryCommandState|queryCommandSupported|queryCommandValue|\n   querySelector|querySelectorAll|registerContentHandler|registerElement|\n   registerProtocolHandler|releaseCapture|releaseEvents|remove|removeAttribute|\n   removeAttributeNode|removeAttributeNS|removeChild|removeEventListener|\n   removeItem|replace|replaceChild|replaceData|replaceWith|reportValidity|\n   requestFullscreen|requestPointerLock|reset|scroll|scrollBy|scrollIntoView|\n   scrollTo|seekToNextFrame|select|selectNode|selectNodeContents|set|setAttribute|\n   setAttributeNode|setAttributeNodeNS|setAttributeNS|setCapture|\n   setCustomValidity|setEnd|setEndAfter|setEndBefore|setItem|setNamedItem|\n   setRangeText|setSelectionRange|setSinkId|setStart|setStartAfter|setStartBefore|\n   slice|splitText|stepDown|stepUp|stopImmediatePropagation|stopPropagation|\n   submit|substringData|supports|surroundContents|takeRecords|terminate|toBlob|\n   toDataURL|toggle|toString|values|write|writeln\n ) |\n (all|catch|finally|race|reject|resolve|then\n )\n)(?=\\s*\\()"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288788234731520,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288788293582999,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 61925255252934807,
            b: 0,
        },
    ]),(4, vec![
        Scope {
            a: 61925255095451648,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 61925255140147351,
            b: 0,
        },
    ]),(6, vec![
        Scope {
            a: 61925255252803735,
            b: 0,
        },
    ])]),
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }