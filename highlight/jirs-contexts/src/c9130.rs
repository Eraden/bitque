
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
    Pattern::Include(ContextReference::Direct(ContextId { index: 9101 })),
    Pattern::Include(ContextReference::Direct(ContextId { index: 9100 })),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("^\\s+(:)(\\b(?x)(\n    display|width|background-color|height|position|font-family|font-weight\n  | top|opacity|cursor|background-image|right|visibility|box-sizing\n  | user-select|left|float|margin-left|margin-top|line-height\n  | padding-left|z-index|margin-bottom|margin-right|margin\n  | vertical-align|padding-top|white-space|border-radius|padding-bottom\n  | padding-right|padding|bottom|clear|max-width|box-shadow|content\n  | border-color|min-height|min-width|font-style|border-width\n  | border-collapse|background-size|text-overflow|max-height|text-transform\n  | text-shadow|text-indent|border-style|overflow-y|list-style-type\n  | word-wrap|border-spacing|appearance|zoom|overflow-x|border-top-left-radius\n  | border-bottom-left-radius|border-top-color|pointer-events\n  | border-bottom-color|align-items|justify-content|letter-spacing\n  | border-top-right-radius|border-bottom-right-radius|border-right-width\n  | font-smoothing|border-bottom-width|border-right-color|direction\n  | border-top-width|src|border-left-color|border-left-width\n  | tap-highlight-color|table-layout|background-clip|word-break\n  | transform-origin|resize|filter|backdrop-filter|backface-visibility|text-rendering\n  | box-orient|transition-property|transition-duration|word-spacing\n  | quotes|outline-offset|animation-timing-function|animation-duration\n  | animation-name|transition-timing-function|border-bottom-style\n  | border-bottom|transition-delay|transition|unicode-bidi|border-top-style\n  | border-top|unicode-range|list-style-position|orphans|outline-width\n  | line-clamp|order|flex-direction|box-pack|animation-fill-mode\n  | outline-color|list-style-image|list-style|touch-action|flex-grow\n  | border-left-style|border-left|animation-iteration-count\n  | page-break-inside|box-flex|box-align|page-break-after|animation-delay\n  | widows|border-right-style|border-right|flex-align|outline-style\n  | outline|background-origin|animation-direction|fill-opacity\n  | background-attachment|flex-wrap|transform-style|counter-increment\n  | overflow-wrap|counter-reset|animation-play-state|animation\n  | will-change|box-ordinal-group|image-rendering|mask-image|flex-flow\n  | background-position-y|stroke-width|background-position-x|background-position\n  | background-blend-mode|flex-shrink|flex-basis|flex-order|flex-item-align\n  | flex-line-pack|flex-negative|flex-pack|flex-positive|flex-preferred-size\n  | flex|user-drag|font-stretch|column-count|empty-cells|align-self\n  | caption-side|mask-size|column-gap|mask-repeat|box-direction\n  | font-feature-settings|mask-position|align-content|object-fit\n  | columns|text-fill-color|clip-path|stop-color|font-kerning\n  | page-break-before|stroke-dasharray|size|fill-rule|border-image-slice\n  | column-width|break-inside|column-break-before|border-image-width\n  | stroke-dashoffset|border-image-repeat|border-image-outset|line-break\n  | stroke-linejoin|stroke-linecap|stroke-miterlimit|stroke-opacity\n  | stroke|shape-rendering|border-image-source|border-image|border\n  | tab-size|writing-mode|perspective-origin-y|perspective-origin-x\n  | perspective-origin|perspective|text-align-last|text-align|clip-rule\n  | clip|text-anchor|column-rule-color|box-decoration-break|column-fill\n  | fill|column-rule-style|mix-blend-mode|text-emphasis-color\n  | baseline-shift|dominant-baseline|page|alignment-baseline\n  | column-rule-width|column-rule|break-after|font-variant-ligatures\n  | transform-origin-y|transform-origin-x|transform|object-position\n  | break-before|column-span|isolation|shape-outside|all\n  | color-interpolation-filters|marker|marker-end|marker-start\n  | marker-mid|color-rendering|color-interpolation|background-repeat-x\n  | background-repeat-y|background-repeat|background|mask-type\n  | flood-color|flood-opacity|text-orientation|mask-composite\n  | text-emphasis-style|paint-order|lighting-color|shape-margin\n  | text-emphasis-position|text-emphasis|shape-image-threshold\n  | mask-clip|mask-origin|mask|font-variant-caps|font-variant-alternates\n  | font-variant-east-asian|font-variant-numeric|font-variant-position\n  | font-variant|font-size-adjust|font-size|font-language-override\n  | font-display|font-synthesis|font|line-box-contain|text-justify\n  | text-decoration-color|text-decoration-style|text-decoration-line\n  | text-decoration|text-underline-position|grid-template-rows\n  | grid-template-columns|grid-template-areas|grid-template|rotate|scale\n  | translate|scroll-behavior|grid-column-start|grid-column-end\n  | grid-column-gap|grid-row-start|grid-row-end|grid-auto-rows\n  | grid-area|grid-auto-flow|grid-auto-columns|image-orientation\n  | hyphens|overflow-scrolling|overflow|color-profile|kerning\n  | nbsp-mode|color|image-resolution|grid-row-gap|grid-row|grid-column\n  | blend-mode|azimuth|pause-after|pause-before|pause|pitch-range|pitch\n  | text-height|system|negative|prefix|suffix|range|pad|fallback\n  | additive-symbols|symbols|speak-as|speak|grid-gap\n)\\b)(?=\\s)"),
      scope: vec![],
      captures: Some(vec![(1, vec![
        Scope {
            a: 47288620737429518,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 46445535821692928,
            b: 0,
        },
        Scope {
            a: 61925375377670158,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9035 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(font-family|font|family)\\b(?=\\s*:)"),
      scope: vec![
        Scope {
            a: 46445535821692928,
            b: 0,
        },
        Scope {
            a: 61925375377670158,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9036 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?x)(\n    display|width|background-color|height|position|font-family|font-weight\n  | top|opacity|cursor|background-image|right|visibility|box-sizing\n  | user-select|left|float|margin-left|margin-top|line-height\n  | padding-left|z-index|margin-bottom|margin-right|margin\n  | vertical-align|padding-top|white-space|border-radius|padding-bottom\n  | padding-right|padding|bottom|clear|max-width|box-shadow|content\n  | border-color|min-height|min-width|font-style|border-width\n  | border-collapse|background-size|text-overflow|max-height|text-transform\n  | text-shadow|text-indent|border-style|overflow-y|list-style-type\n  | word-wrap|border-spacing|appearance|zoom|overflow-x|border-top-left-radius\n  | border-bottom-left-radius|border-top-color|pointer-events\n  | border-bottom-color|align-items|justify-content|letter-spacing\n  | border-top-right-radius|border-bottom-right-radius|border-right-width\n  | font-smoothing|border-bottom-width|border-right-color|direction\n  | border-top-width|src|border-left-color|border-left-width\n  | tap-highlight-color|table-layout|background-clip|word-break\n  | transform-origin|resize|filter|backdrop-filter|backface-visibility|text-rendering\n  | box-orient|transition-property|transition-duration|word-spacing\n  | quotes|outline-offset|animation-timing-function|animation-duration\n  | animation-name|transition-timing-function|border-bottom-style\n  | border-bottom|transition-delay|transition|unicode-bidi|border-top-style\n  | border-top|unicode-range|list-style-position|orphans|outline-width\n  | line-clamp|order|flex-direction|box-pack|animation-fill-mode\n  | outline-color|list-style-image|list-style|touch-action|flex-grow\n  | border-left-style|border-left|animation-iteration-count\n  | page-break-inside|box-flex|box-align|page-break-after|animation-delay\n  | widows|border-right-style|border-right|flex-align|outline-style\n  | outline|background-origin|animation-direction|fill-opacity\n  | background-attachment|flex-wrap|transform-style|counter-increment\n  | overflow-wrap|counter-reset|animation-play-state|animation\n  | will-change|box-ordinal-group|image-rendering|mask-image|flex-flow\n  | background-position-y|stroke-width|background-position-x|background-position\n  | background-blend-mode|flex-shrink|flex-basis|flex-order|flex-item-align\n  | flex-line-pack|flex-negative|flex-pack|flex-positive|flex-preferred-size\n  | flex|user-drag|font-stretch|column-count|empty-cells|align-self\n  | caption-side|mask-size|column-gap|mask-repeat|box-direction\n  | font-feature-settings|mask-position|align-content|object-fit\n  | columns|text-fill-color|clip-path|stop-color|font-kerning\n  | page-break-before|stroke-dasharray|size|fill-rule|border-image-slice\n  | column-width|break-inside|column-break-before|border-image-width\n  | stroke-dashoffset|border-image-repeat|border-image-outset|line-break\n  | stroke-linejoin|stroke-linecap|stroke-miterlimit|stroke-opacity\n  | stroke|shape-rendering|border-image-source|border-image|border\n  | tab-size|writing-mode|perspective-origin-y|perspective-origin-x\n  | perspective-origin|perspective|text-align-last|text-align|clip-rule\n  | clip|text-anchor|column-rule-color|box-decoration-break|column-fill\n  | fill|column-rule-style|mix-blend-mode|text-emphasis-color\n  | baseline-shift|dominant-baseline|page|alignment-baseline\n  | column-rule-width|column-rule|break-after|font-variant-ligatures\n  | transform-origin-y|transform-origin-x|transform|object-position\n  | break-before|column-span|isolation|shape-outside|all\n  | color-interpolation-filters|marker|marker-end|marker-start\n  | marker-mid|color-rendering|color-interpolation|background-repeat-x\n  | background-repeat-y|background-repeat|background|mask-type\n  | flood-color|flood-opacity|text-orientation|mask-composite\n  | text-emphasis-style|paint-order|lighting-color|shape-margin\n  | text-emphasis-position|text-emphasis|shape-image-threshold\n  | mask-clip|mask-origin|mask|font-variant-caps|font-variant-alternates\n  | font-variant-east-asian|font-variant-numeric|font-variant-position\n  | font-variant|font-size-adjust|font-size|font-language-override\n  | font-display|font-synthesis|font|line-box-contain|text-justify\n  | text-decoration-color|text-decoration-style|text-decoration-line\n  | text-decoration|text-underline-position|grid-template-rows\n  | grid-template-columns|grid-template-areas|grid-template|rotate|scale\n  | translate|scroll-behavior|grid-column-start|grid-column-end\n  | grid-column-gap|grid-row-start|grid-row-end|grid-auto-rows\n  | grid-area|grid-auto-flow|grid-auto-columns|image-orientation\n  | hyphens|overflow-scrolling|overflow|color-profile|kerning\n  | nbsp-mode|color|image-resolution|grid-row-gap|grid-row|grid-column\n  | blend-mode|azimuth|pause-after|pause-before|pause|pitch-range|pitch\n  | text-height|system|negative|prefix|suffix|range|pad|fallback\n  | additive-symbols|symbols|speak-as|speak|grid-gap\n)\\b"),
      scope: vec![
        Scope {
            a: 46445535821692928,
            b: 0,
        },
        Scope {
            a: 61925375377670158,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9038 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(-(?:webkit|moz|ms|o)-)((?:--(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+|-?(?:[[_a-zA-Z][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))(?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))*))(?=\\s*:)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445535821692928,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 61925375377735694,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925375377670158,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9039 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(\\$)([a-zA-Z0-9_-][\\w-]*)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 49260513230389248,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629322514573,
            b: 0,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9040 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("(--)((?:[[-\\w][\\p{L}\\p{M}\\p{S}\\p{N}&&[[:^ascii:]]]]|(?:\\\\\\h{1,6}[ \\t\\n\\f]?|\\\\[^\\n\\f\\h]))+)"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445535821692928,
            b: 0,
        },
        Scope {
            a: 61925375377932302,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 47288629344337934,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 61925375377932500,
            b: 3940649673949184,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9041 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(z|y|x|writing|wrap|word|will|width|white|weight|visibility|vertical|variant-position|variant-numeric|variant-ligatures|variant-east-asian|variant-caps|variant-alternates|variant|user|unicode|underline-position|underline|type|transition|transform|touch|top-width|top-style|top-right-radius|top-left-radius|top-color|top|timing-function|timing|threshold|text|template-rows|template-columns|template-areas|template|tap|table|tab|synthesis|symbols|style-type|style-position|style-image|style|stroke|stretch|stop|state|start|speak|span|spacing|space|source|smoothing|slice|sizing|size-adjust|size|side|shrink|shift|shape|shadow|settings|self|select|scrolling|scroll|rule-width|rule-style|rule-color|rule|rows|row-start|row-gap|row-end|row|right-width|right-style|right-radius|right-color|right|resolution|reset|repeat-y|repeat-x|repeat|rendering|range|radius|property|profile|preferred-size|preferred|positive|position-y|position-x|position|pointer|play-state|play|pitch|perspective|pause|path|paint|page|padding|pack|override|overflow|outside|outset|outline|origin-y|origin-x|origin|orientation|orient|ordinal-group|ordinal|order|opacity|offset|object|numeric|negative|nbsp|name|mode|mix|miterlimit|min|mid|max|mask|marker|margin|list|linejoin|linecap|line-pack|line|lighting|ligatures|letter|left-width|left-style|left-radius|left-color|left|layout|last|language-override|language|kerning|justify|iteration-count|iteration|items|item-align|item|interpolation-filters|interpolation|inside|index|indent|increment|image-width|image-threshold|image-source|image-slice|image-repeat|image-outset|image|highlight-color|highlight|height|grow|group|grid|gap|function|font|flow|flood|flex|fit|filters|fill-mode|fill-color|fill|feature-settings|feature|family|events|end|empty|emphasis-style|emphasis-position|emphasis-color|emphasis|duration|drag|dominant|display|direction|delay|decoration-style|decoration-line|decoration-color|decoration-break|decoration|dashoffset|dasharray|counter|count|content|contain|composite|columns|column-start|column-gap|column-end|column|color|collapse|clip|clamp|change|cells|caption|caps|break-inside|break-before|break-after|break|box-contain|box|bottom-width|bottom-style|bottom-color|bottom|border|blend-mode|blend|bidi|behavior|before|basis|baseline|background|backface|auto-rows|auto-flow|auto-columns|auto|attachment|as|areas|area|animation|anchor|alternates|alignment|align-last|align|after|adjust|additive|action)\\b"),
      scope: vec![
        Scope {
            a: 46445535821692928,
            b: 0,
        },
        Scope {
            a: 61925375377670158,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9042 }),
    ]),
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("[a-zA-Z0-9_-]*((#)({)((\\$)([a-zA-Z0-9_-][\\w-]*))(}))[a-zA-Z0-9_-]*"),
      scope: vec![],
      captures: Some(vec![(0, vec![
        Scope {
            a: 46445535821692928,
            b: 0,
        },
        Scope {
            a: 61925375377670144,
            b: 0,
        },
    ]),(1, vec![
        Scope {
            a: 46443865105367181,
            b: 0,
        },
    ]),(2, vec![
        Scope {
            a: 47288629322514573,
            b: 0,
        },
    ]),(3, vec![
        Scope {
            a: 47288629318582454,
            b: 39687971716202496,
        },
    ]),(4, vec![
        Scope {
            a: 49259087301246976,
            b: 0,
        },
    ]),(5, vec![
        Scope {
            a: 47288629322514573,
            b: 0,
        },
    ]),(7, vec![
        Scope {
            a: 47288629318582443,
            b: 39687971716202496,
        },
    ])]),
      operation: MatchOperation::Push(vec![
        ContextReference::Direct(ContextId { index: 9043 }),
    ]),
      with_prototype: None
    }),
]
} }