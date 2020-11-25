use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::i32;
use std::usize;

use fnv::FnvHasher;

use crate::parsing::syntax_set::{SyntaxReference, SyntaxSet};

use super::regex::Region;
use super::scope::*;
use super::syntax_definition::*;

#[derive(Eq, PartialEq)]
pub struct ParseState {
    pub stack: Vec<StateLevel>,
    pub first_line: bool,
    pub proto_starts: Vec<usize>,
}

#[derive(Eq, PartialEq)]
pub struct StateLevel {
    pub context: ContextId,
    pub prototypes: Vec<ContextId>,
    pub captures: Option<(Region, String)>,
}

pub struct RegexMatch<'a> {
    pub regions: Region,
    pub context: &'a Context,
    pub pat_index: usize,
    pub from_with_prototype: bool,
    pub would_loop: bool,
}

type SearchCache<'l> = HashMap<*const MatchPattern, Option<Region>, BuildHasherDefault<FnvHasher>>;

impl ParseState {
    pub fn new(syntax: &SyntaxReference) -> ParseState {
        let start_state = StateLevel {
            context: syntax.contexts["__start"],
            prototypes: Vec::new(),
            captures: None,
        };
        Self {
            stack: vec![start_state],
            first_line: true,
            proto_starts: Vec::new(),
        }
    }

    pub fn parse_line(&mut self, line: &str, syntax_set: &SyntaxSet) -> Vec<(usize, ScopeStackOp)> {
        assert!(
            !self.stack.is_empty(),
            "Somehow main context was popped from the stack"
        );
        let mut match_start = 0;
        let mut res = Vec::new();

        if self.first_line {
            let cur_level = &self.stack[self.stack.len() - 1];
            let context = syntax_set.get_context(&cur_level.context);
            if !context.meta_content_scope.is_empty() {
                res.push((0, ScopeStackOp::Push(context.meta_content_scope[0])));
            }
            self.first_line = false;
        }

        let mut regions = Region::new();
        let fnv = BuildHasherDefault::<FnvHasher>::default();
        let mut search_cache: SearchCache = HashMap::with_capacity_and_hasher(128, fnv);
        // Used for detecting loops with push/pop, see long comment above.
        let mut non_consuming_push_at = (0, 0);

        while self.parse_next_token(
            line,
            syntax_set,
            &mut match_start,
            &mut search_cache,
            &mut regions,
            &mut non_consuming_push_at,
            &mut res,
        ) {}

        res
    }

    fn parse_next_token(
        &mut self,
        line: &str,
        syntax_set: &SyntaxSet,
        start: &mut usize,
        search_cache: &mut SearchCache,
        regions: &mut Region,
        non_consuming_push_at: &mut (usize, usize),
        ops: &mut Vec<(usize, ScopeStackOp)>,
    ) -> bool {
        let check_pop_loop = {
            let (pos, stack_depth) = *non_consuming_push_at;
            pos == *start && stack_depth == self.stack.len()
        };

        // Trim proto_starts that are no longer valid
        while self
            .proto_starts
            .last()
            .map(|start| *start >= self.stack.len())
            .unwrap_or(false)
        {
            self.proto_starts.pop();
        }

        let best_match = self.find_best_match(
            line,
            *start,
            syntax_set,
            search_cache,
            regions,
            check_pop_loop,
        );

        if let Some(reg_match) = best_match {
            if reg_match.would_loop {
                if let Some((i, _)) = line[*start..].char_indices().nth(1) {
                    *start += i;
                    return true;
                } else {
                    // End of line, no character to advance and no point trying
                    // any more patterns.
                    return false;
                }
            }

            let match_end = reg_match.regions.pos(0).unwrap().1;

            let consuming = match_end > *start;
            if !consuming {
                // The match doesn't consume any characters. If this is a
                // "push", remember the position and stack size so that we can
                // check the next "pop" for loops. Otherwise leave the state,
                // e.g. non-consuming "set" could also result in a loop.
                let context = reg_match.context;
                let match_pattern = context.match_at(reg_match.pat_index);
                if let MatchOperation::Push(_) = match_pattern.operation {
                    *non_consuming_push_at = (match_end, self.stack.len() + 1);
                }
            }

            *start = match_end;

            // ignore `with_prototype`s below this if a context is pushed
            if reg_match.from_with_prototype {
                // use current height, since we're before the actual push
                self.proto_starts.push(self.stack.len());
            }

            let level_context = {
                let id = &self.stack[self.stack.len() - 1].context;
                syntax_set.get_context(id)
            };
            self.exec_pattern(line, &reg_match, level_context, syntax_set, ops);

            true
        } else {
            false
        }
    }

    fn find_best_match<'a>(
        &self,
        line: &str,
        start: usize,
        syntax_set: &'a SyntaxSet,
        search_cache: &mut SearchCache,
        regions: &mut Region,
        check_pop_loop: bool,
    ) -> Option<RegexMatch<'a>> {
        let cur_level = &self.stack[self.stack.len() - 1];
        let context = syntax_set.get_context(&cur_level.context);
        let prototype = if let Some(ref p) = context.prototype {
            Some(p)
        } else {
            None
        };

        // Build an iterator for the contexts we want to visit in order
        let context_chain = {
            let proto_start = self.proto_starts.last().cloned().unwrap_or(0);
            // Sublime applies with_prototypes from bottom to top
            let with_prototypes = self.stack[proto_start..].iter().flat_map(|lvl| {
                lvl.prototypes
                    .iter()
                    .map(move |ctx| (true, ctx, lvl.captures.as_ref()))
            });
            let cur_prototype = prototype.into_iter().map(|ctx| (false, ctx, None));
            let cur_context =
                Some((false, &cur_level.context, cur_level.captures.as_ref())).into_iter();
            with_prototypes.chain(cur_prototype).chain(cur_context)
        };

        let mut min_start = usize::MAX;
        let mut best_match: Option<RegexMatch<'_>> = None;
        let mut pop_would_loop = false;

        for (from_with_proto, ctx, captures) in context_chain {
            for (pat_context, pat_index) in context_iter(syntax_set, syntax_set.get_context(ctx)) {
                let match_pat = pat_context.match_at(pat_index);

                if let Some(match_region) =
                    self.search(line, start, match_pat, captures, search_cache, regions)
                {
                    let (match_start, match_end) = match_region.pos(0).unwrap();

                    if match_start < min_start || (match_start == min_start && pop_would_loop) {
                        min_start = match_start;

                        let consuming = match_end > start;
                        pop_would_loop = check_pop_loop
                            && !consuming
                            && match match_pat.operation {
                                MatchOperation::Pop => true,
                                _ => false,
                            };

                        best_match = Some(RegexMatch {
                            regions: match_region.clone(),
                            context: pat_context,
                            pat_index,
                            from_with_prototype: from_with_proto,
                            would_loop: pop_would_loop,
                        });

                        if match_start == start && !pop_would_loop {
                            return best_match;
                        }
                    }
                }
            }
        }
        best_match
    }

    fn search(
        &self,
        line: &str,
        start: usize,
        match_pat: &MatchPattern,
        captures: Option<&(Region, String)>,
        search_cache: &mut SearchCache,
        regions: &mut Region,
    ) -> Option<Region> {
        let match_ptr = match_pat as *const MatchPattern;

        if let Some(maybe_region) = search_cache.get(&match_ptr) {
            if let Some(ref region) = *maybe_region {
                let match_start = region.pos(0).unwrap().0;
                if match_start >= start {
                    return Some(region.clone());
                }
            } else {
                return None;
            }
        }

        let (matched, can_cache) = if match_pat.has_captures && captures.is_some() {
            let &(ref region, ref s) = captures.unwrap();
            let regex = match_pat.regex_with_refs(region, s);
            let matched = regex.search(line, start, line.len(), Some(regions));
            (matched, false)
        } else {
            let regex = match_pat.regex();
            let matched = regex.search(line, start, line.len(), Some(regions));
            (matched, true)
        };

        if matched {
            let (match_start, match_end) = regions.pos(0).unwrap();
            // this is necessary to avoid infinite looping on dumb patterns
            let does_something = match match_pat.operation {
                MatchOperation::None => match_start != match_end,
                _ => true,
            };
            if can_cache && does_something {
                search_cache.insert(match_pat, Some(regions.clone()));
            }
            if does_something {
                // print!("catch {} at {} on {}", match_pat.regex_str, match_start, line);
                return Some(regions.clone());
            }
        } else if can_cache {
            search_cache.insert(match_pat, None);
        }
        None
    }

    /// Returns true if the stack was changed
    fn exec_pattern<'a>(
        &mut self,
        line: &str,
        reg_match: &RegexMatch<'a>,
        level_context: &'a Context,
        syntax_set: &'a SyntaxSet,
        ops: &mut Vec<(usize, ScopeStackOp)>,
    ) -> bool {
        let (match_start, match_end) = reg_match.regions.pos(0).unwrap();
        let context = reg_match.context;
        let pat = context.match_at(reg_match.pat_index);
        // println!("running pattern {:?} on '{}' at {}, operation {:?}", pat.regex_str, line, match_start, pat.operation);

        self.push_meta_ops(
            true,
            match_start,
            level_context,
            &pat.operation,
            syntax_set,
            ops,
        );
        for s in &pat.scope {
            // println!("pushing {:?} at {}", s, match_start);
            ops.push((match_start, ScopeStackOp::Push(*s)));
        }
        if let Some(ref capture_map) = pat.captures {
            // captures could appear in an arbitrary order, have to produce ops in right order
            // ex: ((bob)|(hi))* could match hibob in wrong order, and outer has to push first
            // we don't have to handle a capture matching multiple times, Sublime doesn't
            let mut map: Vec<((usize, i32), ScopeStackOp)> = Vec::new();
            for &(cap_index, ref scopes) in capture_map.iter() {
                if let Some((cap_start, cap_end)) = reg_match.regions.pos(cap_index) {
                    // marking up empty captures causes pops to be sorted wrong
                    if cap_start == cap_end {
                        continue;
                    }
                    // println!("capture {:?} at {:?}-{:?}", scopes[0], cap_start, cap_end);
                    for scope in scopes.iter() {
                        map.push((
                            (cap_start, -((cap_end - cap_start) as i32)),
                            ScopeStackOp::Push(*scope),
                        ));
                    }
                    map.push(((cap_end, i32::MIN), ScopeStackOp::Pop(scopes.len())));
                }
            }
            map.sort_by(|a, b| a.0.cmp(&b.0));
            for ((index, _), op) in map.into_iter() {
                ops.push((index, op));
            }
        }
        if !pat.scope.is_empty() {
            // println!("popping at {}", match_end);
            ops.push((match_end, ScopeStackOp::Pop(pat.scope.len())));
        }
        self.push_meta_ops(
            false,
            match_end,
            &*level_context,
            &pat.operation,
            syntax_set,
            ops,
        );

        self.perform_op(line, &reg_match.regions, pat, syntax_set)
    }

    fn push_meta_ops<'a>(
        &self,
        initial: bool,
        index: usize,
        cur_context: &Context,
        match_op: &MatchOperation,
        syntax_set: &'a SyntaxSet,
        ops: &mut Vec<(usize, ScopeStackOp)>,
    ) {
        match *match_op {
            MatchOperation::Pop => {
                let v = if initial {
                    &cur_context.meta_content_scope
                } else {
                    &cur_context.meta_scope
                };
                if !v.is_empty() {
                    ops.push((index, ScopeStackOp::Pop(v.len())));
                }

                // cleared scopes are restored after the scopes from match pattern that invoked the pop are applied
                if !initial && cur_context.clear_scopes != None {
                    ops.push((index, ScopeStackOp::Restore));
                }
            }
            // for some reason the ST3 behaviour of set is convoluted and is inconsistent with the docs and other ops
            // - the meta_content_scope of the current context is applied to the matched thing, unlike pop
            // - the clear_scopes are applied after the matched token, unlike push
            // - the interaction with meta scopes means that the token has the meta scopes of both the current scope and the new scope.
            MatchOperation::Push(ref context_refs) | MatchOperation::Set(ref context_refs) => {
                let is_set = match *match_op {
                    MatchOperation::Set(_) => true,
                    _ => false,
                };
                // a match pattern that "set"s keeps the meta_content_scope and meta_scope from the previous context
                if initial {
                    if is_set && cur_context.clear_scopes != None {
                        // cleared scopes from the old context are restored immediately
                        ops.push((index, ScopeStackOp::Restore));
                    }
                    // add each context's meta scope
                    for r in context_refs.iter() {
                        let ctx = r.resolve(syntax_set);

                        if !is_set {
                            if let Some(clear_amount) = ctx.clear_scopes {
                                ops.push((index, ScopeStackOp::Clear(clear_amount)));
                            }
                        }

                        for scope in ctx.meta_scope.iter() {
                            ops.push((index, ScopeStackOp::Push(*scope)));
                        }
                    }
                } else {
                    let repush = (is_set
                        && (!cur_context.meta_scope.is_empty()
                            || !cur_context.meta_content_scope.is_empty()))
                        || context_refs.iter().any(|r| {
                            let ctx = r.resolve(syntax_set);

                            !ctx.meta_content_scope.is_empty()
                                || (ctx.clear_scopes.is_some() && is_set)
                        });
                    if repush {
                        // remove previously pushed meta scopes, so that meta content scopes will be applied in the correct order
                        let mut num_to_pop: usize = context_refs
                            .iter()
                            .map(|r| {
                                let ctx = r.resolve(syntax_set);
                                ctx.meta_scope.len()
                            })
                            .sum();

                        // also pop off the original context's meta scopes
                        if is_set {
                            num_to_pop +=
                                cur_context.meta_content_scope.len() + cur_context.meta_scope.len();
                        }

                        // do all the popping as one operation
                        if num_to_pop > 0 {
                            ops.push((index, ScopeStackOp::Pop(num_to_pop)));
                        }

                        // now we push meta scope and meta context scope for each context pushed
                        for r in context_refs {
                            let ctx = r.resolve(syntax_set);

                            // for some reason, contrary to my reading of the docs, set does this after the token
                            if is_set {
                                if let Some(clear_amount) = ctx.clear_scopes {
                                    ops.push((index, ScopeStackOp::Clear(clear_amount)));
                                }
                            }

                            for scope in ctx.meta_scope.iter() {
                                ops.push((index, ScopeStackOp::Push(*scope)));
                            }
                            for scope in ctx.meta_content_scope.iter() {
                                ops.push((index, ScopeStackOp::Push(*scope)));
                            }
                        }
                    }
                }
            }
            MatchOperation::None => (),
        }
    }

    /// Returns true if the stack was changed
    fn perform_op(
        &mut self,
        line: &str,
        regions: &Region,
        pat: &MatchPattern,
        syntax_set: &SyntaxSet,
    ) -> bool {
        let (ctx_refs, old_proto_ids) = match pat.operation {
            MatchOperation::Push(ref ctx_refs) => (ctx_refs, None),
            MatchOperation::Set(ref ctx_refs) => {
                // a `with_prototype` stays active when the context is `set`
                // until the context layer in the stack (where the `with_prototype`
                // was initially applied) is popped off.
                (ctx_refs, self.stack.pop().map(|s| s.prototypes))
            }
            MatchOperation::Pop => {
                self.stack.pop();
                return true;
            }
            MatchOperation::None => return false,
        };
        for (i, r) in ctx_refs.iter().enumerate() {
            let mut proto_ids = if i == 0 {
                // it is only necessary to preserve the old prototypes
                // at the first stack frame pushed
                old_proto_ids.clone().unwrap_or_else(|| Vec::new())
            } else {
                Vec::new()
            };
            if i == ctx_refs.len() - 1 {
                // if a with_prototype was specified, and multiple contexts were pushed,
                // then the with_prototype applies only to the last context pushed, i.e.
                // top most on the stack after all the contexts are pushed - this is also
                // referred to as the "target" of the push by sublimehq - see
                // https://forum.sublimetext.com/t/dev-build-3111/19240/17 for more info
                if let Some(ref p) = pat.with_prototype {
                    proto_ids.push(p.id().clone());
                }
            }
            let context_id = r.id();
            let context = syntax_set.get_context(&context_id);
            let captures = {
                let mut uses_backrefs = context.uses_backrefs;
                if !proto_ids.is_empty() {
                    uses_backrefs = uses_backrefs
                        || proto_ids
                            .iter()
                            .any(|id| syntax_set.get_context(id).uses_backrefs);
                }
                if uses_backrefs {
                    Some((regions.clone(), line.to_owned()))
                } else {
                    None
                }
            };
            self.stack.push(StateLevel {
                context: context_id,
                prototypes: proto_ids,
                captures,
            });
        }
        true
    }
}
