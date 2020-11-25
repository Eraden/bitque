//! This module contains data structures for representing syntax definitions.
//! Everything is public because I want this library to be useful in super
//! integrated cases like text editors and I have no idea what kind of monkeying
//! you might want to do with the data. Perhaps parsing your own syntax format
//! into this data structure?
use super::regex::{Regex, Region};
use super::scope::*;
use crate::syntect::parsing::syntax_set::SyntaxSet;
use regex_syntax::escape;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub type CaptureMapping = Vec<(usize, Vec<Scope>)>;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContextId {
    pub index: usize,
}

/// The main data structure representing a syntax definition loaded from a
/// `.sublime-syntax` file. You'll probably only need these as references
/// to be passed around to parsing code.
///
/// Some useful public fields are the `name` field which is a human readable
/// name to display in syntax lists, and the `hidden` field which means hide
/// this syntax from any lists because it is for internal use.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SyntaxDefinition {
    pub name: String,
    pub file_extensions: Vec<String>,
    pub scope: Scope,
    pub first_line_match: Option<String>,
    pub hidden: bool,
    pub variables: HashMap<String, String>,
    pub contexts: HashMap<String, Context>,
}

#[derive(Clone, Eq, PartialEq, Default, Deserialize)]
pub struct Context {
    pub meta_scope: Vec<Scope>,
    pub meta_content_scope: Vec<Scope>,
    pub meta_include_prototype: bool,
    pub clear_scopes: Option<ClearAmount>,
    pub prototype: Option<ContextId>,
    pub uses_backrefs: bool,
    pub patterns: Vec<Pattern>,
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Context {\n")?;
        f.write_str(format!("  meta_scope: vec!{:#?},\n", self.meta_content_scope).as_str())?;
        f.write_str(
            format!(
                "  meta_content_scope: vec!{:#?},\n",
                self.meta_content_scope
            )
            .as_str(),
        )?;
        f.write_str(
            format!(
                "  meta_include_prototype: {:#?},\n",
                self.meta_include_prototype
            )
            .as_str(),
        )?;
        f.write_str(format!("  clear_scopes: {:#?},\n", self.clear_scopes).as_str())?;
        f.write_str(format!("  prototype: {:#?},\n", self.prototype).as_str())?;
        f.write_str(format!("  uses_backrefs: {:#?},\n", self.uses_backrefs).as_str())?;
        f.write_str(format!("  patterns: vec!{:#?}\n", self.patterns).as_str())?;
        f.write_str("}")
    }
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

#[derive(Clone, Eq, PartialEq, Deserialize)]
pub enum Pattern {
    Match(MatchPattern),
    Include(ContextReference),
}

impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Match(a) => f.write_str(format!("Pattern::Match({:#?})", a).as_str()),
            Pattern::Include(a) => f.write_str(format!("Pattern::Include({:#?})", a).as_str()),
        }
    }
}

pub struct MatchIter<'a> {
    pub syntax_set: &'a SyntaxSet,
    pub ctx_stack: Vec<&'a Context>,
    pub index_stack: Vec<usize>,
}

impl<'l> std::fmt::Debug for MatchIter<'l> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MatchIter {\n")?;
        f.write_str(format!("  syntax_set: {:#?},\n", self.syntax_set).as_str())?;
        f.write_str(format!("  ctx_stack: vec!{:#?},\n", self.ctx_stack).as_str())?;
        f.write_str(format!("  index_stack: vec!{:#?}\n", self.index_stack).as_str())?;
        f.write_str("}")
    }
}

#[derive(Clone, Eq, PartialEq, Deserialize)]
pub struct MatchPattern {
    pub has_captures: bool,
    pub regex: Regex,
    pub scope: Vec<Scope>,
    pub captures: Option<CaptureMapping>,
    pub operation: MatchOperation,
    pub with_prototype: Option<ContextReference>,
}

impl std::fmt::Debug for MatchPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MatchPattern {\n")?;
        f.write_str(format!("  has_captures: {:#?},\n", self.has_captures).as_str())?;
        f.write_str(format!("  regex: {:#?},\n", self.regex).as_str())?;
        f.write_str(format!("  scope: vec!{:#?},\n", self.scope).as_str())?;
        f.write_str(
            format!(
                "  captures: {},\n",
                self.captures
                    .as_ref()
                    .map(|scope| {
                        let s = scope
                            .iter()
                            .map(|(idx, v)| format!("({}, vec!{:#?})", idx, v))
                            .collect::<Vec<String>>()
                            .join(",");
                        format!("Some(vec![{}])", s)
                    })
                    .unwrap_or_else(|| "None".to_string())
            )
            .as_str(),
        )?;
        f.write_str(format!("  operation: {:#?},\n", self.operation).as_str())?;
        f.write_str(format!("  with_prototype: {:#?}\n", self.with_prototype).as_str())?;
        f.write_str("}")
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
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

impl std::fmt::Debug for ContextReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextReference::Named(inner) => {
                f.write_str(format!("ContextReference::Named({:?}.to_string())", inner).as_str())
            }
            ContextReference::ByScope { scope, sub_context } => f.write_str(
                format!(
                    "ContextReference::ByScope {{ scope: {:#?}, sub_context: {}  }}",
                    scope,
                    sub_context
                        .as_ref()
                        .map(|s| format!("Some({:?}.to_string())", s))
                        .unwrap_or_else(|| "None".to_string())
                )
                .as_str(),
            ),
            ContextReference::File { name, sub_context } => f.write_str(
                format!(
                    "ContextReference::File {{ name: {:#?}.to_string(), sub_context: {} }}",
                    name,
                    sub_context
                        .as_ref()
                        .map(|s| format!("Some({:?}.to_string())", s))
                        .unwrap_or_else(|| "None".to_string())
                )
                .as_str(),
            ),
            ContextReference::Inline(inner) => {
                f.write_str(format!("ContextReference::Inline({:?}.to_string())", inner).as_str())
            }
            ContextReference::Direct(inner) => {
                f.write_str(format!("ContextReference::Direct({:?})", inner).as_str())
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MatchOperation {
    Push(Vec<ContextReference>),
    Set(Vec<ContextReference>),
    Pop,
    None,
}

impl std::fmt::Debug for MatchOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchOperation::Push(inner) => {
                f.write_str(format!("MatchOperation::Push(vec!{:#?})", inner).as_str())
            }
            MatchOperation::Set(inner) => {
                f.write_str(format!("MatchOperation::Set(vec!{:#?})", inner).as_str())
            }
            MatchOperation::Pop => f.write_str(format!("MatchOperation::Pop").as_str()),
            MatchOperation::None => f.write_str(format!("MatchOperation::None").as_str()),
        }
    }
}

impl<'a> Iterator for MatchIter<'a> {
    type Item = (&'a Context, usize);

    fn next(&mut self) -> Option<(&'a Context, usize)> {
        loop {
            if self.ctx_stack.is_empty() {
                return None;
            }
            // uncomment for debugging infinite recursion
            // println!("{:?}", self.index_stack);
            // use std::thread::sleep_ms;
            // sleep_ms(500);
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

/// Returns an iterator over all the match patterns in this context.
/// It recursively follows include directives. Can only be run on
/// contexts that have already been linked up.
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
        regex_str: &str,
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

    /// Used by the parser to compile a regex which needs to reference
    /// regions from another matched pattern.
    pub fn regex_with_refs(&self, region: &Region, text: &str) -> Regex {
        let new_regex = substitute_backrefs_in_regex(self.regex.regex_str(), |i| {
            region.pos(i).map(|(start, end)| escape(&text[start..end]))
        });

        return Regex::new(new_regex.as_str());
    }

    pub fn regex(&self) -> &Regex {
        &self.regex
    }
}

/// Serialize the provided map in natural key order, so that it's deterministic when dumping.
pub fn ordered_map<K, V, S>(map: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    K: Eq + Hash + Ord + Serialize,
    V: Serialize,
{
    let ordered: BTreeMap<_, _> = map.iter().collect();
    ordered.serialize(serializer)
}
