//! This module contains data structures for representing syntax definitions.
//! Everything is public because I want this library to be useful in super
//! integrated cases like text editors and I have no idea what kind of monkeying
//! you might want to do with the data. Perhaps parsing your own syntax format
//! into this data structure?
use super::regex::{Regex, Region};
use super::scope::*;
use crate::parsing::syntax_set::SyntaxSet;
use regex_syntax::escape;
use std::collections::HashMap;

pub type CaptureMapping = Vec<(usize, Vec<Scope>)>;

#[derive(serde::Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContextId {
    pub index: usize,
}

pub struct SyntaxDefinition {
    pub name: String,
    pub file_extensions: Vec<String>,
    pub scope: Scope,
    pub first_line_match: Option<String>,
    pub hidden: bool,
    pub variables: HashMap<String, String>,
    pub contexts: HashMap<String, Context>,
}

#[derive(serde::Deserialize, Default)]
pub struct Context {
    pub meta_scope: Vec<Scope>,
    pub meta_content_scope: Vec<Scope>,
    pub meta_include_prototype: bool,
    pub clear_scopes: Option<ClearAmount>,
    pub prototype: Option<ContextId>,
    pub uses_backrefs: bool,
    pub patterns: Vec<Pattern>,
}

impl Context {
    pub fn new(meta_include_prototype: bool) -> Context {
        Context {
            meta_scope: Vec::new(),
            meta_content_scope: Vec::new(),
            meta_include_prototype,
            clear_scopes: None,
            uses_backrefs: false,
            patterns: Vec::new(),
            prototype: None,
        }
    }
}

#[derive(serde::Deserialize)]
pub enum Pattern {
    Match(MatchPattern),
    Include(ContextReference),
}

pub struct MatchIter<'a> {
    pub syntax_set: &'a SyntaxSet,
    pub ctx_stack: Vec<&'a Context>,
    pub index_stack: Vec<usize>,
}

#[derive(serde::Deserialize)]
pub struct MatchPattern {
    pub has_captures: bool,
    pub regex: Regex,
    pub scope: Vec<Scope>,
    pub captures: Option<CaptureMapping>,
    pub operation: MatchOperation,
    pub with_prototype: Option<ContextReference>,
}

#[derive(serde::Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum ContextReference {
    Named(String),
    ByScope {
        scope: Scope,
        sub_context: Option<String>,
    },
    File {
        name: String,
        sub_context: Option<String>,
    },
    Inline(String),
    Direct(ContextId),
}

#[derive(serde::Deserialize, Clone, Eq, PartialEq)]
pub enum MatchOperation {
    Push(Vec<ContextReference>),
    Set(Vec<ContextReference>),
    Pop,
    None,
}

impl<'a> Iterator for MatchIter<'a> {
    type Item = (&'a Context, usize);

    fn next(&mut self) -> Option<(&'a Context, usize)> {
        loop {
            if self.ctx_stack.is_empty() {
                return None;
            }
            let last_index = self.ctx_stack.len() - 1;
            let context = self.ctx_stack[last_index];
            let index = self.index_stack[last_index];
            self.index_stack[last_index] = index + 1;
            if index < context.patterns.len() {
                match context.patterns[index] {
                    Pattern::Match(_) => {
                        return Some((context, index));
                    }
                    Pattern::Include(ref ctx_ref) => {
                        let ctx_ptr = match *ctx_ref {
                            ContextReference::Direct(ref context_id) => {
                                self.syntax_set.get_context(context_id)
                            }
                            _ => return self.next(), // skip this and move onto the next one
                        };
                        self.ctx_stack.push(ctx_ptr);
                        self.index_stack.push(0);
                    }
                }
            } else {
                self.ctx_stack.pop();
                self.index_stack.pop();
            }
        }
    }
}

pub fn context_iter<'a>(syntax_set: &'a SyntaxSet, context: &'a Context) -> MatchIter<'a> {
    MatchIter {
        syntax_set,
        ctx_stack: vec![context],
        index_stack: vec![0],
    }
}

impl Context {
    /// Returns the match pattern at an index, panics if the thing isn't a match pattern
    pub fn match_at(&self, index: usize) -> &MatchPattern {
        match self.patterns[index] {
            Pattern::Match(ref match_pat) => match_pat,
            _ => panic!("bad index to match_at"),
        }
    }
}

impl ContextReference {
    /// find the pointed to context, panics if ref is not linked
    pub fn resolve<'a>(&self, syntax_set: &'a SyntaxSet) -> &'a Context {
        match *self {
            ContextReference::Direct(ref context_id) => syntax_set.get_context(context_id),
            _ => panic!("Can only call resolve on linked references: {:?}", self),
        }
    }

    /// get the context ID this reference points to, panics if ref is not linked
    pub fn id(&self) -> ContextId {
        match *self {
            ContextReference::Direct(ref context_id) => *context_id,
            _ => panic!("Can only get ContextId of linked references: {:?}", self),
        }
    }
}

pub fn substitute_backrefs_in_regex<F>(regex_str: &str, substituter: F) -> String
where
    F: Fn(usize) -> Option<String>,
{
    let mut reg_str = String::with_capacity(regex_str.len());

    let mut last_was_escape = false;
    for c in regex_str.chars() {
        if last_was_escape && c.is_digit(10) {
            let val = c.to_digit(10).unwrap() as usize;
            if let Some(sub) = substituter(val) {
                reg_str.push_str(&sub);
            }
        } else if last_was_escape {
            reg_str.push('\\');
            reg_str.push(c);
        } else if c != '\\' {
            reg_str.push(c);
        }

        last_was_escape = c == '\\' && !last_was_escape;
    }
    reg_str
}

impl ContextId {
    pub fn new(index: usize) -> Self {
        ContextId { index }
    }

    #[inline(always)]
    pub(crate) fn index(self) -> usize {
        self.index
    }
}

impl MatchPattern {
    pub fn new(
        has_captures: bool,
        regex_str: &'static str,
        scope: Vec<Scope>,
        captures: Option<CaptureMapping>,
        operation: MatchOperation,
        with_prototype: Option<ContextReference>,
    ) -> Self {
        Self {
            has_captures,
            regex: Regex::new(regex_str),
            scope,
            captures,
            operation,
            with_prototype,
        }
    }

    pub fn regex_with_refs(&self, region: &Region, text: &str) -> Regex {
        let new_regex = substitute_backrefs_in_regex(self.regex.regex_str(), |i| {
            region.pos(i).map(|(start, end)| escape(&text[start..end]))
        });

        Regex::new_with_owned(new_regex)
    }

    pub fn regex(&self) -> &Regex {
        &self.regex
    }
}
