use crate::syntect::parsing::{MatchPower, ParseScopeError, Scope, ScopeStack};
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ScopeSelector {
    pub path: ScopeStack,
    pub excludes: Vec<ScopeStack>,
}

impl std::fmt::Debug for ScopeSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "ScopeSelector::new({:#?}, vec!{:#?})",
                self.path, self.excludes
            )
            .as_str(),
        )
    }
}

#[derive(Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ScopeSelectors {
    pub selectors: Vec<ScopeSelector>,
}

impl std::fmt::Debug for ScopeSelectors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("ScopeSelectors::new(vec!{:#?})", self.selectors).as_str())
    }
}

impl ScopeSelector {
    pub fn does_match(&self, stack: &[Scope]) -> Option<MatchPower> {
        // if there are any exclusions, and any one of them matches, then this selector doesn't match
        if self
            .excludes
            .iter()
            .any(|sel| sel.is_empty() || sel.does_match(stack).is_some())
        {
            return None;
        }
        if self.path.is_empty() {
            // an empty scope selector always matches with a score of 1
            Some(MatchPower(0o1u64 as f64))
        } else {
            self.path.does_match(stack)
        }
    }

    /// If this selector is really just a single scope, return it
    pub fn extract_single_scope(&self) -> Option<Scope> {
        if self.path.len() > 1 || !self.excludes.is_empty() || self.path.is_empty() {
            return None;
        }
        Some(self.path.as_slice()[0])
    }

    /// extract all selectors for generating css
    pub fn extract_scopes(&self) -> Vec<Scope> {
        self.path.scopes.clone()
    }
}

impl FromStr for ScopeSelector {
    type Err = ParseScopeError;

    /// Parses a scope stack followed optionally by (one or more) " -" and then a scope stack to exclude
    fn from_str(s: &str) -> Result<ScopeSelector, ParseScopeError> {
        let mut excludes = Vec::new();
        let mut path_str: &str = "";
        for (i, selector) in s.split(" -").enumerate() {
            if i == 0 {
                path_str = selector;
            } else {
                excludes.push(ScopeStack::from_str(selector)?);
            }
        }
        Ok(ScopeSelector {
            path: ScopeStack::from_str(path_str)?,
            excludes,
        })
    }
}

impl ScopeSelectors {
    pub fn does_match(&self, stack: &[Scope]) -> Option<MatchPower> {
        self.selectors
            .iter()
            .filter_map(|sel| sel.does_match(stack))
            .max()
    }
}

impl FromStr for ScopeSelectors {
    type Err = ParseScopeError;
    fn from_str(s: &str) -> Result<ScopeSelectors, ParseScopeError> {
        let mut selectors = Vec::new();
        for selector in s.split(&[',', '|'][..]) {
            selectors.push(ScopeSelector::from_str(selector)?)
        }
        Ok(ScopeSelectors { selectors })
    }
}
