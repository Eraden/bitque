use std::collections::{HashMap, HashSet};
use std::io;
use std::mem;
use std::path::Path;

use crate::parsing::syntax_definition::ContextId;

use super::regex::Regex;
use super::scope::*;
use super::syntax_definition::*;

#[derive(serde::Deserialize)]
pub struct SyntaxSet {
    pub syntaxes: Vec<SyntaxReference>,
    pub contexts: Vec<Context>,
}

#[derive(serde::Deserialize)]
pub struct SyntaxReference {
    pub name: String,
    pub file_extensions: Vec<String>,
    pub scope: Scope,
    pub first_line_match: Option<String>,
    pub hidden: bool,
    pub variables: HashMap<String, String>,
    pub contexts: HashMap<String, ContextId>,
}

#[derive(Default)]
pub struct SyntaxSetBuilder {
    pub syntaxes: Vec<SyntaxDefinition>,
}

impl Default for SyntaxSet {
    fn default() -> Self {
        Self {
            syntaxes: vec![],
            contexts: vec![],
        }
    }
}

impl SyntaxSet {
    pub fn new() -> Self {
        SyntaxSet::default()
    }
    pub fn syntaxes(&self) -> &[SyntaxReference] {
        &self.syntaxes[..]
    }
    pub fn find_syntax_by_scope(&self, scope: Scope) -> Option<&SyntaxReference> {
        self.syntaxes.iter().rev().find(|&s| s.scope == scope)
    }
    pub fn find_syntax_by_name<'a>(&'a self, name: &str) -> Option<&'a SyntaxReference> {
        self.syntaxes.iter().rev().find(|&s| name == s.name)
    }
    pub fn find_syntax_by_extension<'a>(&'a self, extension: &str) -> Option<&'a SyntaxReference> {
        self.syntaxes
            .iter()
            .rev()
            .find(|&s| s.file_extensions.iter().any(|e| e == extension))
    }
    pub fn find_syntax_by_token<'a>(&'a self, s: &str) -> Option<&'a SyntaxReference> {
        {
            let ext_res = self.find_syntax_by_extension(s);
            if ext_res.is_some() {
                return ext_res;
            }
        }
        self.syntaxes
            .iter()
            .rev()
            .find(|&syntax| syntax.name.eq_ignore_ascii_case(s))
    }

    pub fn find_syntax_for_file<P: AsRef<Path>>(
        &self,
        path_obj: P,
    ) -> io::Result<Option<&SyntaxReference>> {
        let path: &Path = path_obj.as_ref();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let extension = path.extension().and_then(|x| x.to_str()).unwrap_or("");
        let ext_syntax = self
            .find_syntax_by_extension(file_name)
            .or_else(|| self.find_syntax_by_extension(extension));
        Ok(ext_syntax)
    }
    pub fn find_syntax_plain_text(&self) -> &SyntaxReference {
        self.find_syntax_by_name("Plain Text")
            .expect("All syntax sets ought to have a plain text syntax")
    }
    pub fn into_builder(self) -> SyntaxSetBuilder {
        let SyntaxSet {
            syntaxes, contexts, ..
        } = self;

        let mut context_map = HashMap::with_capacity(contexts.len());
        for (i, context) in contexts.into_iter().enumerate() {
            context_map.insert(i, context);
        }

        let mut builder_syntaxes = Vec::with_capacity(syntaxes.len());

        for syntax in syntaxes {
            let SyntaxReference {
                name,
                file_extensions,
                scope,
                first_line_match,
                hidden,
                variables,
                contexts,
            } = syntax;

            let mut builder_contexts = HashMap::with_capacity(contexts.len());
            for (name, context_id) in contexts {
                if let Some(context) = context_map.remove(&context_id.index()) {
                    builder_contexts.insert(name, context);
                }
            }

            let syntax_definition = SyntaxDefinition {
                name,
                file_extensions,
                scope,
                first_line_match,
                hidden,
                variables,
                contexts: builder_contexts,
            };
            builder_syntaxes.push(syntax_definition);
        }

        SyntaxSetBuilder {
            syntaxes: builder_syntaxes,
        }
    }

    #[inline(always)]
    pub fn get_context(&self, context_id: &ContextId) -> &Context {
        &self.contexts[context_id.index()]
    }
}

impl SyntaxSetBuilder {
    pub fn new() -> Self {
        SyntaxSetBuilder::default()
    }

    pub fn add(&mut self, syntax: SyntaxDefinition) {
        self.syntaxes.push(syntax);
    }

    pub fn build(self) -> SyntaxSet {
        let SyntaxSetBuilder {
            syntaxes: syntax_definitions,
        } = self;

        let mut syntaxes = Vec::with_capacity(syntax_definitions.len());
        let mut all_contexts = Vec::new();

        for syntax_definition in syntax_definitions {
            let SyntaxDefinition {
                name,
                file_extensions,
                scope,
                first_line_match,
                hidden,
                variables,
                contexts,
            } = syntax_definition;

            let mut map = HashMap::new();

            let mut contexts: Vec<(String, Context)> = contexts.into_iter().collect();
            // Sort the values of the HashMap so that the contexts in the
            // resulting SyntaxSet have a deterministic order for serializing.
            // Because we're sorting by the keys which are unique, we can use
            // an unstable sort.
            contexts.sort_unstable_by(|(name_a, _), (name_b, _)| name_a.cmp(&name_b));
            for (name, context) in contexts {
                let index = all_contexts.len();
                map.insert(name, ContextId::new(index));
                all_contexts.push(context);
            }

            let syntax = SyntaxReference {
                name,
                file_extensions,
                scope,
                first_line_match,
                hidden,
                variables,
                contexts: map,
            };
            syntaxes.push(syntax);
        }

        let mut found_more_backref_includes = true;
        for syntax in &syntaxes {
            let mut no_prototype = HashSet::new();
            let prototype = syntax.contexts.get("prototype");
            if let Some(prototype_id) = prototype {
                // TODO: We could do this after parsing YAML, instead of here?
                Self::recursively_mark_no_prototype(
                    syntax,
                    prototype_id.index(),
                    &all_contexts,
                    &mut no_prototype,
                );
            }

            for context_id in syntax.contexts.values() {
                let index = context_id.index();
                let mut context = &mut all_contexts[index];
                if let Some(prototype_id) = prototype {
                    if context.meta_include_prototype && !no_prototype.contains(&index) {
                        context.prototype = Some(*prototype_id);
                    }
                }
                Self::link_context(&mut context, syntax, &syntaxes);

                if context.uses_backrefs {
                    found_more_backref_includes = true;
                }
            }
        }

        // We need to recursively mark contexts that include contexts which
        // use backreferences as using backreferences. In theory we could use
        // a more efficient method here like doing a toposort or constructing
        // a representation with reversed edges and then tracing in the
        // opposite direction, but I benchmarked this and it adds <2% to link
        // time on the default syntax set, and linking doesn't even happen
        // when loading from a binary dump.
        while found_more_backref_includes {
            found_more_backref_includes = false;
            // find any contexts which include a context which uses backrefs
            // and mark those as using backrefs - to support nested includes
            for context_index in 0..all_contexts.len() {
                let context = &all_contexts[context_index];
                if !context.uses_backrefs
                    && context.patterns.iter().any(|pattern| match pattern {
                        Pattern::Include(ContextReference::Direct(id))
                            if all_contexts[id.index()].uses_backrefs =>
                        {
                            true
                        }
                        _ => false,
                    })
                {
                    let mut context = &mut all_contexts[context_index];
                    context.uses_backrefs = true;
                    // look for contexts including this context
                    found_more_backref_includes = true;
                }
            }
        }

        #[cfg(feature = "metadata")]
        let metadata = match existing_metadata {
            Some(existing) => existing.merged_with_raw(raw_metadata),
            None => raw_metadata.into(),
        };

        SyntaxSet {
            syntaxes,
            contexts: all_contexts,
        }
    }

    /// Anything recursively included by the prototype shouldn't include the prototype.
    /// This marks them as such.
    fn recursively_mark_no_prototype(
        syntax: &SyntaxReference,
        context_id: usize,
        contexts: &[Context],
        no_prototype: &mut HashSet<usize>,
    ) {
        let first_time = no_prototype.insert(context_id);
        if !first_time {
            return;
        }

        for pattern in &contexts[context_id].patterns {
            match *pattern {
                // Apparently inline blocks also don't include the prototype when within the prototype.
                // This is really weird, but necessary to run the YAML syntax.
                Pattern::Match(ref match_pat) => {
                    let maybe_context_refs = match match_pat.operation {
                        MatchOperation::Push(ref context_refs)
                        | MatchOperation::Set(ref context_refs) => Some(context_refs),
                        MatchOperation::Pop | MatchOperation::None => None,
                    };
                    if let Some(context_refs) = maybe_context_refs {
                        for context_ref in context_refs.iter() {
                            match context_ref {
                                ContextReference::Inline(ref s)
                                | ContextReference::Named(ref s) => {
                                    if let Some(i) = syntax.contexts.get(s) {
                                        Self::recursively_mark_no_prototype(
                                            syntax,
                                            i.index(),
                                            contexts,
                                            no_prototype,
                                        );
                                    }
                                }
                                ContextReference::Direct(ref id) => {
                                    Self::recursively_mark_no_prototype(
                                        syntax,
                                        id.index(),
                                        contexts,
                                        no_prototype,
                                    );
                                }
                                _ => (),
                            }
                        }
                    }
                }
                Pattern::Include(ref reference) => match reference {
                    ContextReference::Named(ref s) => {
                        if let Some(id) = syntax.contexts.get(s) {
                            Self::recursively_mark_no_prototype(
                                syntax,
                                id.index(),
                                contexts,
                                no_prototype,
                            );
                        }
                    }
                    ContextReference::Direct(ref id) => {
                        Self::recursively_mark_no_prototype(
                            syntax,
                            id.index(),
                            contexts,
                            no_prototype,
                        );
                    }
                    _ => (),
                },
            }
        }
    }

    fn link_context(context: &mut Context, syntax: &SyntaxReference, syntaxes: &[SyntaxReference]) {
        for pattern in &mut context.patterns {
            match *pattern {
                Pattern::Match(ref mut match_pat) => {
                    Self::link_match_pat(match_pat, syntax, syntaxes)
                }
                Pattern::Include(ref mut context_ref) => {
                    Self::link_ref(context_ref, syntax, syntaxes)
                }
            }
        }
    }

    fn link_ref(
        context_ref: &mut ContextReference,
        syntax: &SyntaxReference,
        syntaxes: &[SyntaxReference],
    ) {
        // println!("{:?}", context_ref);
        use super::syntax_definition::ContextReference::*;
        let linked_context_id = match *context_ref {
            Named(ref s) | Inline(ref s) => {
                // This isn't actually correct, but it is better than nothing/crashing.
                // This is being phased out anyhow, see https://github.com/sublimehq/Packages/issues/73
                // Fixes issue #30
                if s == "$top_level_main" {
                    syntax.contexts.get("main")
                } else {
                    syntax.contexts.get(s)
                }
            }
            ByScope {
                scope,
                ref sub_context,
            } => {
                let context_name = sub_context.as_ref().map_or("main", |x| &**x);
                syntaxes
                    .iter()
                    .rev()
                    .find(|s| s.scope == scope)
                    .and_then(|s| s.contexts.get(context_name))
            }
            File {
                ref name,
                ref sub_context,
            } => {
                let context_name = sub_context.as_ref().map_or("main", |x| &**x);
                syntaxes
                    .iter()
                    .rev()
                    .find(|s| &s.name == name)
                    .and_then(|s| s.contexts.get(context_name))
            }
            Direct(_) => None,
        };
        if let Some(context_id) = linked_context_id {
            let mut new_ref = Direct(*context_id);
            mem::swap(context_ref, &mut new_ref);
        }
    }

    fn link_match_pat(
        match_pat: &mut MatchPattern,
        syntax: &SyntaxReference,
        syntaxes: &[SyntaxReference],
    ) {
        let maybe_context_refs = match match_pat.operation {
            MatchOperation::Push(ref mut context_refs)
            | MatchOperation::Set(ref mut context_refs) => Some(context_refs),
            MatchOperation::Pop | MatchOperation::None => None,
        };
        if let Some(context_refs) = maybe_context_refs {
            for context_ref in context_refs.iter_mut() {
                Self::link_ref(context_ref, syntax, syntaxes);
            }
        }
        if let Some(ref mut context_ref) = match_pat.with_prototype {
            Self::link_ref(context_ref, syntax, syntaxes);
        }
    }
}

// pub struct FirstLineCache {
//     pub regexes: Vec<(Regex, usize)>,
// }
//
// impl FirstLineCache {
//     fn new(syntaxes: &[SyntaxReference]) -> Self {
//         let mut regexes = Vec::new();
//         for (i, syntax) in syntaxes.iter().enumerate() {
//             if let Some(ref reg_str) = syntax.first_line_match {
//                 let reg = Regex::new(reg_str.as_str());
//                 regexes.push((reg, i));
//             }
//         }
//         Self { regexes }
//     }
// }
