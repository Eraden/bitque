use super::super::LoadingError;
use super::settings::*;
use super::theme::Theme;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ThemeSet {
    pub themes: BTreeMap<String, Theme>,
}

/// A set of themes, includes convenient methods for loading and discovering themes.
impl ThemeSet {
    /// Creates an empty set
    pub fn new() -> ThemeSet {
        ThemeSet::default()
    }

    /// Loads a theme given a path to a .tmTheme file
    pub fn get_theme<P: AsRef<Path>>(path: P) -> Result<Theme, LoadingError> {
        let file = File::open(path)?;
        let mut file = BufReader::new(file);
        Self::load_from_reader(&mut file)
    }

    /// Loads a theme given a readable stream
    pub fn load_from_reader<R: BufRead + Seek>(r: &mut R) -> Result<Theme, LoadingError> {
        Ok(Theme::parse_settings(read_plist(r)?)?)
    }
}
