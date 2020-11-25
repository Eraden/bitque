/// Code based on https://github.com/defuz/sublimate/blob/master/src/core/syntax/theme.rs
/// released under the MIT license by @defuz
use std::str::FromStr;

use super::selector::*;
use super::style::*;
use crate::parsing::ParseScopeError;

use self::ParseThemeError::*;

/// A theme parsed from a `.tmTheme` file.
/// Contains fields useful for a theme list as well as `settings` for styling your editor.
#[derive(Clone, Default)]
pub struct Theme {
    pub name: Option<String>,
    pub author: Option<String>,
    pub settings: ThemeSettings,
    pub scopes: Vec<ThemeItem>,
}

impl Theme {
    pub fn new(
        name: Option<&str>,
        author: Option<&str>,
        settings: ThemeSettings,
        scopes: Vec<ThemeItem>,
    ) -> Self {
        Self {
            name: name.map(String::from),
            author: author.map(String::from),
            settings,
            scopes,
        }
    }
}

/// Various properties meant to be used to style a text editor.
/// Basically all the styles that aren't directly applied to text like selection color.
/// Use this to make your editor UI match the highlighted text.
#[derive(Clone, Debug, Default)]
pub struct ThemeSettings {
    /// The default color for text.
    pub foreground: Option<Color>,
    /// The default backgound color of the view.
    pub background: Option<Color>,
    /// Color of the caret.
    pub caret: Option<Color>,
    /// Color of the line the caret is in.
    /// Only used when the `higlight_line` setting is set to `true`.
    pub line_highlight: Option<Color>,

    /// The color to use for the squiggly underline drawn under misspelled words.
    pub misspelling: Option<Color>,
    /// The color of the border drawn around the viewport area of the minimap.
    /// Only used when the `draw_minimap_border` setting is enabled.
    pub minimap_border: Option<Color>,
    /// A color made available for use by the theme.
    pub accent: Option<Color>,
    /// CSS passed to popups.
    pub popup_css: Option<String>,
    /// CSS passed to phantoms.
    pub phantom_css: Option<String>,

    /// Color of bracketed sections of text when the caret is in a bracketed section.
    /// Only applied when the `match_brackets` setting is set to `true`.
    pub bracket_contents_foreground: Option<Color>,
    /// Controls certain options when the caret is in a bracket section.
    /// Only applied when the `match_brackets` setting is set to `true`.
    pub bracket_contents_options: Option<UnderlineOption>,
    /// Foreground color of the brackets when the caret is next to a bracket.
    /// Only applied when the `match_brackets` setting is set to `true`.
    pub brackets_foreground: Option<Color>,
    /// Background color of the brackets when the caret is next to a bracket.
    /// Only applied when the `match_brackets` setting is set to `true`.
    pub brackets_background: Option<Color>,
    /// Controls certain options when the caret is next to a bracket.
    /// Only applied when the match_brackets setting is set to `true`.
    pub brackets_options: Option<UnderlineOption>,

    /// Color of tags when the caret is next to a tag.
    /// Only used when the `match_tags` setting is set to `true`.
    pub tags_foreground: Option<Color>,
    /// Controls certain options when the caret is next to a tag.
    /// Only applied when the match_tags setting is set to `true`.
    pub tags_options: Option<UnderlineOption>,

    /// The border color for "other" matches.
    pub highlight: Option<Color>,
    /// Background color of regions matching the current search.
    pub find_highlight: Option<Color>,
    /// Text color of regions matching the current search.
    pub find_highlight_foreground: Option<Color>,

    /// Background color of the gutter.
    pub gutter: Option<Color>,
    /// Foreground color of the gutter.
    pub gutter_foreground: Option<Color>,

    /// The background color of selected text.
    pub selection: Option<Color>,
    /// A color that will override the scope-based text color of the selection.
    pub selection_foreground: Option<Color>,

    /// Deprecated!
    ///
    /// This property is not part of the recognized tmTheme format. It may be
    /// removed in a future release.
    pub selection_background: Option<Color>,

    /// Color of the selection regions border.
    pub selection_border: Option<Color>,
    /// The background color of a selection in a view that is not currently focused.
    pub inactive_selection: Option<Color>,
    /// A color that will override the scope-based text color of the selection
    /// in a view that is not currently focused.
    pub inactive_selection_foreground: Option<Color>,

    /// Color of the guides displayed to indicate nesting levels.
    pub guide: Option<Color>,
    /// Color of the guide lined up with the caret.
    /// Only applied if the `indent_guide_options` setting is set to `draw_active`.
    pub active_guide: Option<Color>,
    /// Color of the current guide’s parent guide level.
    /// Only used if the `indent_guide_options` setting is set to `draw_active`.
    pub stack_guide: Option<Color>,

    /// Foreground color for regions added via `sublime.add_regions()`
    /// with the `sublime.DRAW_OUTLINED` flag added.
    ///
    /// Deprecated!
    /// This setting does not exist in any available documentation.
    /// Use is discouraged, and it may be removed in a future release.
    pub highlight_foreground: Option<Color>,

    /// The color of the shadow used when a text area can be horizontally scrolled.
    pub shadow: Option<Color>,
}

#[derive(Clone, Debug, Default)]
pub struct ThemeItem {
    pub scope: ScopeSelectors,
    pub style: StyleModifier,
}

#[derive(Clone, Copy)]
pub enum UnderlineOption {
    None,
    Underline,
    StippledUnderline,
    SquigglyUnderline,
}

impl std::fmt::Debug for UnderlineOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnderlineOption::None => f.write_str("UnderlineOption::None"),
            UnderlineOption::Underline => f.write_str("UnderlineOption::Underline"),
            UnderlineOption::StippledUnderline => f.write_str("UnderlineOption::StippledUnderline"),
            UnderlineOption::SquigglyUnderline => f.write_str("UnderlineOption::SquigglyUnderline"),
        }
    }
}

#[derive(Debug)]
pub enum ParseThemeError {
    IncorrectUnderlineOption,
    IncorrectFontStyle(String),
    IncorrectColor,
    IncorrectSyntax,
    IncorrectSettings,
    UndefinedSettings,
    UndefinedScopeSettings(String),
    ColorShemeScopeIsNotObject,
    ColorShemeSettingsIsNotObject,
    ScopeSelectorIsNotString(String),
    DuplicateSettings,
    ScopeParse(ParseScopeError),
}

impl From<ParseScopeError> for ParseThemeError {
    fn from(error: ParseScopeError) -> ParseThemeError {
        ScopeParse(error)
    }
}

impl Default for UnderlineOption {
    fn default() -> UnderlineOption {
        UnderlineOption::None
    }
}

impl Default for FontStyle {
    fn default() -> FontStyle {
        FontStyle::empty()
    }
}

impl FromStr for UnderlineOption {
    type Err = ParseThemeError;

    fn from_str(s: &str) -> Result<UnderlineOption, Self::Err> {
        Ok(match s {
            "underline" => UnderlineOption::Underline,
            "stippled_underline" => UnderlineOption::StippledUnderline,
            "squiggly_underline" => UnderlineOption::SquigglyUnderline,
            _ => return Err(IncorrectUnderlineOption),
        })
    }
}

impl FromStr for FontStyle {
    type Err = ParseThemeError;

    fn from_str(s: &str) -> Result<FontStyle, Self::Err> {
        let mut font_style = FontStyle::empty();
        for i in s.split_whitespace() {
            font_style.insert(match i {
                "bold" => FontStyle::BOLD,
                "underline" => FontStyle::UNDERLINE,
                "italic" => FontStyle::ITALIC,
                "normal" | "regular" => FontStyle::empty(),
                s => return Err(IncorrectFontStyle(s.to_owned())),
            })
        }
        Ok(font_style)
    }
}

impl FromStr for Color {
    type Err = ParseThemeError;

    fn from_str(s: &str) -> Result<Color, Self::Err> {
        let mut chars = s.chars();
        if chars.next() != Some('#') {
            return Err(IncorrectColor);
        }
        let mut d = Vec::new();
        for char in chars {
            d.push(char.to_digit(16).ok_or(IncorrectColor)? as u8);
        }
        Ok(match d.len() {
            3 => Color {
                r: d[0],
                g: d[1],
                b: d[2],
                a: 255,
            },
            6 => Color {
                r: d[0] * 16 + d[1],
                g: d[2] * 16 + d[3],
                b: d[4] * 16 + d[5],
                a: 255,
            },
            8 => Color {
                r: d[0] * 16 + d[1],
                g: d[2] * 16 + d[3],
                b: d[4] * 16 + d[5],
                a: d[6] * 16 + d[7],
            },
            _ => return Err(IncorrectColor),
        })
    }
}
