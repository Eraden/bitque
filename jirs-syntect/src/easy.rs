//! API wrappers for common use cases like highlighting strings and
//! files without caring about intermediate semantic representation
//! and caching.

use crate::highlighting::{HighlightIterator, HighlightState, Highlighter, Style, Theme};
use crate::parsing::{ParseState, ScopeStack, ScopeStackOp, SyntaxReference, SyntaxSet};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
// use util::debug_print_ops;

pub struct HighlightLines<'a> {
    highlighter: Highlighter<'a>,
    parse_state: ParseState,
    highlight_state: HighlightState,
}

impl<'a> HighlightLines<'a> {
    pub fn new(syntax: &SyntaxReference, theme: &'a Theme) -> HighlightLines<'a> {
        let highlighter = Highlighter::new(theme);
        let highlight_state = HighlightState::new(&highlighter, ScopeStack::new());
        HighlightLines {
            highlighter,
            parse_state: ParseState::new(syntax),
            highlight_state,
        }
    }

    /// Highlights a line of a file
    pub fn highlight<'b>(
        &mut self,
        line: &'b str,
        syntax_set: &SyntaxSet,
    ) -> Vec<(Style, &'b str)> {
        let ops = self.parse_state.parse_line(line, syntax_set);
        let iter =
            HighlightIterator::new(&mut self.highlight_state, &ops[..], line, &self.highlighter);
        iter.collect()
    }
}

/// Convenience struct containing everything you need to highlight a file.
/// Use the `reader` to get the lines of the file and the `highlight_lines` to highlight them.
/// See the `new` method docs for more information.
pub struct HighlightFile<'a> {
    pub reader: BufReader<File>,
    pub highlight_lines: HighlightLines<'a>,
}

impl<'a> HighlightFile<'a> {
    pub fn new<P: AsRef<Path>>(
        path_obj: P,
        ss: &SyntaxSet,
        theme: &'a Theme,
    ) -> io::Result<HighlightFile<'a>> {
        let path: &Path = path_obj.as_ref();
        let f = File::open(path)?;
        let syntax = ss
            .find_syntax_for_file(path)?
            .unwrap_or_else(|| ss.find_syntax_plain_text());

        Ok(HighlightFile {
            reader: BufReader::new(f),
            highlight_lines: HighlightLines::new(syntax, theme),
        })
    }
}

#[derive(Debug)]
pub struct ScopeRegionIterator<'a> {
    ops: &'a [(usize, ScopeStackOp)],
    line: &'a str,
    index: usize,
    last_str_index: usize,
}

impl<'a> ScopeRegionIterator<'a> {
    pub fn new(ops: &'a [(usize, ScopeStackOp)], line: &'a str) -> ScopeRegionIterator<'a> {
        ScopeRegionIterator {
            ops,
            line,
            index: 0,
            last_str_index: 0,
        }
    }
}

static NOOP_OP: ScopeStackOp = ScopeStackOp::Noop;
impl<'a> Iterator for ScopeRegionIterator<'a> {
    type Item = (&'a str, &'a ScopeStackOp);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.ops.len() {
            return None;
        }

        // region extends up to next operation (ops[index]) or string end if there is none
        // note the next operation may be at, last_str_index, in which case the region is empty
        let next_str_i = if self.index == self.ops.len() {
            self.line.len()
        } else {
            self.ops[self.index].0
        };
        let substr = &self.line[self.last_str_index..next_str_i];
        self.last_str_index = next_str_i;

        // the first region covers everything before the first op, which may be empty
        let op = if self.index == 0 {
            &NOOP_OP
        } else {
            &self.ops[self.index - 1].1
        };

        self.index += 1;
        Some((substr, op))
    }
}
