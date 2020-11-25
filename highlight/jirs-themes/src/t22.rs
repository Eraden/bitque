use jirs_syntect::*;
use jirs_syntect::highlighting::*;
pub fn load() -> Theme {
Theme::new(  Some(
    "zenburn",
),
  Some(
    "Jani Nurminen.  Adapted and modified by Colin T.A. Gray and William D. Neumann",
),
  ThemeSettings {
    foreground: Some(
        Color {
            r: 222,
            g: 222,
            b: 222,
            a: 255,
        },
    ),
    background: Some(
        Color {
            r: 57,
            g: 57,
            b: 57,
            a: 255,
        },
    ),
    caret: Some(
        Color {
            r: 214,
            g: 214,
            b: 214,
            a: 255,
        },
    ),
    line_highlight: Some(
        Color {
            r: 48,
            g: 48,
            b: 48,
            a: 255,
        },
    ),
    misspelling: None,
    minimap_border: None,
    accent: None,
    popup_css: None,
    phantom_css: None,
    bracket_contents_foreground: None,
    bracket_contents_options: None,
    brackets_foreground: None,
    brackets_background: None,
    brackets_options: None,
    tags_foreground: None,
    tags_options: None,
    highlight: None,
    find_highlight: None,
    find_highlight_foreground: None,
    gutter: None,
    gutter_foreground: None,
    selection: Some(
        Color {
            r: 131,
            g: 131,
            b: 131,
            a: 156,
        },
    ),
    selection_foreground: None,
    selection_background: None,
    selection_border: None,
    inactive_selection: None,
    inactive_selection_foreground: None,
    guide: None,
    active_guide: None,
    stack_guide: None,
    highlight_foreground: None,
    shadow: None,
},
  vec![
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 51509920738050048,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 135,
                g: 174,
                b: 134,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444466373918720,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629327757312,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 232,
                g: 188,
                b: 146,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636787086327808,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 86,
                g: 142,
                b: 77,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629323038720,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288466114281472,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 160,
                g: 207,
                b: 161,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288521949642752,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 137,
                g: 137,
                b: 137,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844708410753024,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 204,
                g: 148,
                b: 149,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636787052118016,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 222,
                g: 222,
                b: 222,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636787052118587,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(2)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955200831520768,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 128,
                b: 128,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450570411999232,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451949096501248,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 214,
                g: 134,
                b: 134,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55451949170753536,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 214,
                g: 214,
                b: 214,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925409718403072,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955089162371072,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 135,
                g: 214,
                b: 213,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59954170039369728,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925409704378368,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 214,
                g: 214,
                b: 174,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955200831520768,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955136407011328,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 204,
                g: 148,
                b: 149,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259087305965568,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 214,
                g: 214,
                b: 174,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52635820644892672,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636636688678912,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445565886464000,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392186477182976,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 254,
                g: 214,
                b: 175,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49259087310291329,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 254,
                g: 214,
                b: 175,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130669019136,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925461293989888,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 128,
                b: 0,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628098744320,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 236,
                g: 236,
                b: 236,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844648268431360,
                    b: 0,
                },
                Scope {
                    a: 52636628137083654,
                    b: 301178225080401920,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(2)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844648268431360,
                    b: 0,
                },
                Scope {
                    a: 52636628137083389,
                    b: 301178225080401920,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, FontStyle::from_bits(2)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414576462528512,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414439023575040,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925375344640000,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 251,
                b: 157,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632320044,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 64,
                g: 128,
                b: 160,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48413695994232832,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632450048,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392186453590016,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 214,
                g: 214,
                b: 214,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392186470432768,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 215,
                g: 141,
                b: 27,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 48414576603168768,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444831446138880,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 224,
                b: 0,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632450048,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632450251,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392186477183179,
                    b: 3940649673949184,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925366754705408,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632318976,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 244,
                g: 160,
                b: 32,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 49258876838608896,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(None, None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 50102545854496768,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 204,
                b: 238,
                a: 255,
            },
        ), None, FontStyle::from_bits(7)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392186477182981,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 214,
                g: 215,
                b: 175,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632122368,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 214,
                g: 215,
                b: 175,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288521949642934,
                    b: 16325548649218048,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288521949642923,
                    b: 16325548649218048,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 137,
                g: 137,
                b: 137,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925255159676928,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 254,
                g: 214,
                b: 175,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629322514432,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 254,
                g: 214,
                b: 175,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925255085555712,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46454078510727168,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130630615040,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 253,
                b: 135,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 61925255085555712,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 199,
                g: 186,
                b: 24,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46445243762999296,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 120,
                g: 206,
                b: 204,
                a: 128,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629323956406,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629323956395,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 214,
                g: 214,
                b: 214,
                a: 128,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629324153014,
                    b: 1407374883553280,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629324153003,
                    b: 1407374883553280,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46444230198624458,
                    b: 1407374883553280,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 254,
                g: 213,
                b: 174,
                a: 148,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59392130632122449,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 254,
                g: 214,
                b: 175,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629366816768,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 214,
                g: 214,
                b: 214,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636787102908485,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636787185877061,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 183,
                g: 183,
                b: 183,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844721282875392,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955136498040901,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 59955136498106437,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 52636628189511680,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 149,
                g: 191,
                b: 243,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844506534510592,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        ), Some(
            Color {
                r: 57,
                g: 57,
                b: 57,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844506534510592,
                    b: 0,
                },
                Scope {
                    a: 46443452773629952,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 844506534510592,
                    b: 0,
                },
                Scope {
                    a: 46445273839763475,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        ), Some(
            Color {
                r: 159,
                g: 157,
                b: 21,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443452799386177,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 202,
                g: 113,
                b: 114,
                a: 255,
            },
        ), Some(
            Color {
                r: 57,
                g: 57,
                b: 57,
                a: 255,
            },
        ), FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443452799386178,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 96,
                g: 179,
                b: 138,
                a: 255,
            },
        ), Some(
            Color {
                r: 57,
                g: 57,
                b: 57,
                a: 255,
            },
        ), FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 46443452779528766,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 140,
                g: 208,
                b: 211,
                a: 255,
            },
        ), Some(
            Color {
                r: 57,
                g: 57,
                b: 57,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281327331835904,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 96,
                g: 179,
                b: 138,
                a: 255,
            },
        ), Some(
            Color {
                r: 57,
                g: 57,
                b: 57,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281335921770496,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 202,
                g: 113,
                b: 114,
                a: 255,
            },
        ), Some(
            Color {
                r: 57,
                g: 57,
                b: 57,
                a: 255,
            },
        ), None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450759390560256,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 199,
                g: 111,
                b: 65,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450759530545152,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 158,
                g: 106,
                b: 95,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629451030528,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 94,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 55450759445348352,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 203,
                g: 142,
                b: 129,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629365833728,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 94,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280120494129201,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 254,
                g: 214,
                b: 175,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114281636571447296,
                    b: 0,
                },
            ]), vec![]),
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 47288629353709568,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        ), None, FontStyle::from_bits(1)),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 114280017365565440,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 222,
                g: 222,
                b: 222,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 737757188273471488,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 221,
                g: 183,
                b: 0,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 737757192542748672,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        ), None, None),
    },
    ThemeItem {
        scope: ScopeSelectors::new(vec![
            ScopeSelector::new(ScopeStack::new_with_args(vec![], vec![
                Scope {
                    a: 737757188265410560,
                    b: 0,
                },
            ]), vec![]),
        ]),
        style: StyleModifier::new(Some(
            Color {
                r: 208,
                g: 32,
                b: 0,
                a: 255,
            },
        ), None, None),
    },
]
)}
