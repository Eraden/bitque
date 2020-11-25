use std::collections::BTreeMap;

use super::theme::Theme;

#[derive(Default)]
pub struct ThemeSet {
    pub themes: BTreeMap<String, Theme>,
}

/// A set of themes, includes convenient methods for loading and discovering themes.
impl ThemeSet {
    /// Creates an empty set
    pub fn new() -> Self {
        ThemeSet::default()
    }
}
