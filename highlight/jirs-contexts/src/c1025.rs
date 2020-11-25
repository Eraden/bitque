
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
      regex: Regex::new("(?x)\\b(\n    absolute|active|add\n  | all(-(petite|small)-caps|-scroll)?\n  | alpha(betic)?\n  | alternate(-reverse)?\n  | always|annotation|antialiased|at\n  | auto(hiding-scrollbar|-flow)?\n  | avoid(-column|-page|-region)?\n  | background(-color|-image|-position|-size)?\n  | backwards|balance|baseline|below|bevel|bicubic|bidi-override|blink\n  | block(-line-height)?\n  | blur\n  | bold(er)?\n  | border(-bottom|-left|-right|-top)?-(color|radius|width|style)\n  | border-(bottom|top)-(left|right)-radius\n  | border-image(-outset|-repeat|-slice|-source|-width)?\n  | border(-bottom|-left|-right|-top|-collapse|-spacing|-box)?\n  | both|bottom\n  | box(-shadow)?\n  | break-(all|word)\n  | brightness\n  | butt(on)?\n  | capitalize\n  | cent(er|ral)\n  | char(acter-variant)?\n  | cjk-ideographic|clip|clone|close-quote\n  | closest-(corner|side)\n  | col-resize|collapse\n  | color(-stop|-burn|-dodge)?\n  | column((-count|-gap|-reverse|-rule(-color|-width)?|-width)|s)?\n  | common-ligatures|condensed|consider-shifts|contain\n  | content(-box|s)?\n  | contextual|contrast|cover\n  | crisp(-e|E)dges\n  | crop\n  | cross(hair)?\n  | da(rken|shed)\n  | default|dense|diagonal-fractions|difference|disabled\n  | discretionary-ligatures|disregard-shifts\n  | distribute(-all-lines|-letter|-space)?\n  | dotted|double|drop-shadow\n  | (nwse|nesw|ns|ew|sw|se|nw|ne|w|s|e|n)-resize\n  | ease(-in-out|-in|-out)?\n  | element|ellipsis|embed|end|EndColorStr|evenodd\n  | exclu(de(-ruby)?|sion)\n  | expanded\n  | (extra|semi|ultra)-(condensed|expanded)\n  | farthest-(corner|side)?\n  | fill(-box|-opacity)?\n  | filter|first|fixed|flat\n  | fit-content\n  | flex((-basis|-end|-grow|-shrink|-start)|box)?\n  | flip|flood-color\n  | font(-size(-adjust)?|-stretch|-weight)?\n  | forwards\n  | from(-image)?\n  | full-width|geometricPrecision|glyphs|gradient|grayscale\n  | grid(-height)?\n  | groove|hand|hanging|hard-light|height|help|hidden|hide\n  | historical-(forms|ligatures)\n  | horizontal(-tb)?\n  | hue\n  | ideograph(-alpha|-numeric|-parenthesis|-space|ic)\n  | inactive|include-ruby|infinite|inherit|initial\n  | inline(-block|-box|-flex(box)?|-line-height|-table)?\n  | inset|inside\n  | inter(-ideograph|-word|sect)\n  | invert|isolat(e|ion)|italic\n  | jis(04|78|83|90)\n  | justify(-all)?\n  | keep-all\n  | large[r]?\n  | last|left|letter-spacing\n  | light(e[nr]|ing-color)\n  | line(-edge|-height|-through)?\n  | linear(-gradient|RGB)?\n  | lining-nums|list-item|local|loose|lowercase|lr-tb|ltr\n  | lumin(osity|ance)|manual\n  | margin(-bottom|-box|-left|-right|-top)?\n  | marker(-offset|s)?\n  | mathematical\n  | max-(content|height|lines|size|width)\n  | medium|middle\n  | min-(content|height|width)\n  | miter|mixed|move|multiply|newspaper\n  | no-(change|clip|(close|open)-quote|(common|discretionary|historical)-ligatures|contextual|drop|repeat)\n  | none|nonzero|normal|not-allowed|nowrap|oblique\n  | offset(-after|-before|-end|-start)?\n  | oldstyle-nums|opacity|open-quote\n  | optimize(Legibility|Precision|Quality|Speed)\n  | order|ordinal|ornaments\n  | outline(-color|-offset|-width)?\n  | outset|outside|over(line|-edge|lay)\n  | padding(-bottom|-box|-left|-right|-top)?\n  | page|painted|paused\n  | perspective-origin\n  | petite-caps|pixelated|pointer\n  | pre(-line|-wrap)?\n  | preserve-3d\n  | progid:DXImageTransform.Microsoft.(Alpha|Blur|dropshadow|gradient|Shadow)\n  | progress\n  | proportional-(nums|width)\n  | radial-gradient|recto|region|relative\n  | repeat(-[xy])?\n  | repeating-(linear|radial)-gradient\n  | replaced|reset-size|reverse|ridge|right\n  | round\n  | row(-resize|-reverse)?\n  | run-in\n  | ruby(-base|-text)?(-container)?\n  | rtl|running|saturat(e|ion)|screen\n  | safe\n  | scroll(-position|bar)?\n  | separate|sepia\n  | scale-down\n  | shape-(image-threshold|margin|outside)\n  | show\n  | sideways(-lr|-rl)?\n  | simplified\n  | slashed-zero|slice\n  | small(-caps|er)?\n  | smooth|snap|solid|soft-light\n  | space(-around|-between|-evenly)?\n  | span|sRGB\n  | stack(ed-fractions)?\n  | start(ColorStr)?\n  | static\n  | step-(end|start)\n  | sticky\n  | stop-(color|opacity)\n  | stretch|strict\n  | stroke(-box|-dash(array|offset)|-miterlimit|-opacity|-width)?\n  | style(set)?\n  | stylistic\n  | sub(grid|pixel-antialiased|tract)?\n  | super|swash\n  | table(-caption|-cell|(-column|-footer|-header|-row)-group|-column|-row)?\n  | tabular-nums|tb-rl\n  | text((-bottom|-(decoration|emphasis)-color|-indent|-(over|under|after|before)-edge|-shadow|-size(-adjust)?|-top)|field)?\n  | thi(ck|n)\n  | titling-ca(ps|se)\n  | to[p]?\n  | touch|traditional\n  | transform(-origin)?\n  | under(-edge|line)?\n  | unicase|unsafe|unset|uppercase|upright\n  | use-(glyph-orientation|script)\n  | verso\n  | vertical(-align|-ideographic|-lr|-rl|-text)?\n  | view-box\n  | viewport-fill(-opacity)?\n  | visibility\n  | visible(Fill|Painted|Stroke)?\n  | wait|wavy|weight|whitespace|width|word-spacing\n  | wrap(-reverse)?\n  | x{1,2}-(large|small)\n  | z-index|zero\n  | zoom(-in|-out)?\n  | ((?xi:\n    arabic-indic | armenian | bengali | cambodian | circle\n  | cjk-decimal | cjk-earthly-branch | cjk-heavenly-stem | decimal-leading-zero\n  | decimal | devanagari | disclosure-closed | disclosure-open | disc\n  | ethiopic-numeric | georgian | gujarati | gurmukhi | hebrew\n  | hiragana-iroha | hiragana | japanese-formal | japanese-informal\n  | kannada | katakana-iroha | katakana | khmer\n  | korean-hangul-formal | korean-hanja-formal | korean-hanja-informal | lao\n  | lower-alpha | lower-armenian | lower-greek | lower-latin | lower-roman\n  | malayalam | mongolian | myanmar | oriya | persian\n  | simp-chinese-formal | simp-chinese-informal\n  | square | tamil | telugu | thai | tibetan\n  | trad-chinese-formal | trad-chinese-informal\n  | upper-alpha | upper-armenian | upper-latin | upper-roman\n))\n)\\b"),
      scope: vec![
        Scope {
            a: 61925409737015310,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
    Pattern::Match(MatchPattern {
      has_captures: false,
      regex: Regex::new("\\b(?i:sans-serif|serif|monospace|fantasy|cursive|system-ui)\\b(?=\\s*[;,\\n}])"),
      scope: vec![
        Scope {
            a: 61925409739767822,
            b: 0,
        },
    ],
      captures: None,
      operation: MatchOperation::None,
      with_prototype: None
    }),
]
} }