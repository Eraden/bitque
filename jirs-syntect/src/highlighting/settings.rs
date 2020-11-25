use plist::Error as PlistError;
/// Code based on https://github.com/defuz/sublimate/blob/master/src/core/settings.rs
/// released under the MIT license by @defuz

#[derive(Debug)]
pub enum SettingsError {
    Plist(PlistError),
}

impl From<PlistError> for SettingsError {
    fn from(error: PlistError) -> SettingsError {
        SettingsError::Plist(error)
    }
}
