use std::str::FromStr;

/// Code based on https://github.com/defuz/sublimate/blob/master/src/core/syntax/scope.rs
/// released under the MIT license by @defuz
use crate::parsing::{MatchPower, ParseScopeError, Scope, ScopeStack};

/// A single selector consisting of a stack to match and a possible stack to exclude from being matched.
/// You probably want `ScopeSelectors` which is this but with union support.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScopeSelector {
    pub path: ScopeStack,
    pub excludes: Vec<ScopeStack>,
}

/// A selector set that matches anything matched by any of its component selectors.
/// See [The TextMate Docs](https://manual.macromates.com/en/scope_selectors) for how these
/// work.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScopeSelectors {
    /// the selectors, if any of them match, this matches
    pub selectors: Vec<ScopeSelector>,
}

impl ScopeSelectors {
    pub fn new(selectors: Vec<ScopeSelector>) -> Self {
        Self { selectors }
    }
}

impl ScopeSelector {
    pub fn new(path: ScopeStack, excludes: Vec<ScopeStack>) -> Self {
        Self { path, excludes }
    }

    /// Checks if this selector matches a given scope stack.
    /// See `ScopeSelectors#does_match` for more info.
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
    /// checks if any of these selectors match the given scope stack
    /// if so it returns a match score, higher match scores are stronger
    /// matches. Scores are ordered according to the rules found
    /// at https://manual.macromates.com/en/scope_selectors
    pub fn does_match(&self, stack: &[Scope]) -> Option<MatchPower> {
        self.selectors
            .iter()
            .filter_map(|sel| sel.does_match(stack))
            .max()
    }
}

impl FromStr for ScopeSelectors {
    type Err = ParseScopeError;

    /// Parses a series of selectors separated by commas or pipes
    fn from_str(s: &str) -> Result<ScopeSelectors, ParseScopeError> {
        let mut selectors = Vec::new();
        for selector in s.split(&[',', '|'][..]) {
            selectors.push(ScopeSelector::from_str(selector)?)
        }
        Ok(ScopeSelectors { selectors })
    }
}
