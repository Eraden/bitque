use std::iter::Iterator;
use std::ops::Range;

use super::selector::ScopeSelector;
use super::style::{Color, FontStyle, Style, StyleModifier};
use super::theme::{Theme, ThemeItem};
use crate::parsing::{
    BasicScopeStackOp, MatchPower, Scope, ScopeStack, ScopeStackOp, ATOM_LEN_BITS,
};

pub struct Highlighter<'a> {
    pub theme: &'a Theme,
    pub single_selectors: Vec<(Scope, StyleModifier)>,
    pub multi_selectors: Vec<(ScopeSelector, StyleModifier)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightState {
    pub styles: Vec<Style>,
    pub single_caches: Vec<ScoredStyle>,
    pub path: ScopeStack,
}

pub struct RangedHighlightIterator<'a, 'b> {
    pub index: usize,
    pub pos: usize,
    pub changes: &'a [(usize, ScopeStackOp)],
    pub text: &'b str,
    pub highlighter: &'a Highlighter<'a>,
    pub state: &'a mut HighlightState,
}

pub struct HighlightIterator<'a, 'b> {
    ranged_iterator: RangedHighlightIterator<'a, 'b>,
}

impl HighlightState {
    pub fn new(highlighter: &Highlighter<'_>, initial_stack: ScopeStack) -> Self {
        let mut styles = vec![highlighter.get_default()];
        let mut single_caches = vec![ScoredStyle::from_style(styles[0])];
        for i in 0..initial_stack.len() {
            let prefix = initial_stack.bottom_n(i + 1);
            let new_cache = highlighter.update_single_cache_for_push(&single_caches[i], prefix);
            styles.push(highlighter.finalize_style_with_multis(&new_cache, prefix));
            single_caches.push(new_cache);
        }

        Self {
            styles,
            single_caches,
            path: initial_stack,
        }
    }
}

impl<'a, 'b> RangedHighlightIterator<'a, 'b> {
    pub fn new(
        state: &'a mut HighlightState,
        changes: &'a [(usize, ScopeStackOp)],
        text: &'b str,
        highlighter: &'a Highlighter<'_>,
    ) -> RangedHighlightIterator<'a, 'b> {
        RangedHighlightIterator {
            index: 0,
            pos: 0,
            changes,
            text,
            highlighter,
            state,
        }
    }
}

impl<'a, 'b> Iterator for RangedHighlightIterator<'a, 'b> {
    type Item = (Style, &'b str, Range<usize>);

    fn next(&mut self) -> Option<(Style, &'b str, Range<usize>)> {
        if self.pos == self.text.len() && self.index >= self.changes.len() {
            return None;
        }
        let (end, command) = if self.index < self.changes.len() {
            self.changes[self.index].clone()
        } else {
            (self.text.len(), ScopeStackOp::Noop)
        };
        // println!("{} - {:?}   {}:{}", self.index, self.pos, self.state.path.len(), self.state.styles.len());
        let style = *self.state.styles.last().unwrap_or(&Style::default());
        let text = &self.text[self.pos..end];
        let range = Range {
            start: self.pos,
            end: end,
        };
        {
            // closures mess with the borrow checker's ability to see different struct fields
            let m_path = &mut self.state.path;
            let m_styles = &mut self.state.styles;
            let m_caches = &mut self.state.single_caches;
            let highlighter = &self.highlighter;
            m_path.apply_with_hook(&command, |op, cur_stack| {
                // println!("{:?} - {:?}", op, cur_stack);
                match op {
                    BasicScopeStackOp::Push(_) => {
                        // we can push multiple times so this might have changed
                        let new_cache = {
                            if let Some(prev_cache) = m_caches.last() {
                                highlighter.update_single_cache_for_push(prev_cache, cur_stack)
                            } else {
                                highlighter.update_single_cache_for_push(
                                    &ScoredStyle::from_style(highlighter.get_default()),
                                    cur_stack,
                                )
                            }
                        };
                        m_styles
                            .push(highlighter.finalize_style_with_multis(&new_cache, cur_stack));
                        m_caches.push(new_cache);
                    }
                    BasicScopeStackOp::Pop => {
                        m_styles.pop();
                        m_caches.pop();
                    }
                }
            });
        }
        self.pos = end;
        self.index += 1;
        if text.is_empty() {
            self.next()
        } else {
            Some((style, text, range))
        }
    }
}
impl<'a, 'b> HighlightIterator<'a, 'b> {
    pub fn new(
        state: &'a mut HighlightState,
        changes: &'a [(usize, ScopeStackOp)],
        text: &'b str,
        highlighter: &'a Highlighter<'_>,
    ) -> HighlightIterator<'a, 'b> {
        HighlightIterator {
            ranged_iterator: RangedHighlightIterator {
                index: 0,
                pos: 0,
                changes,
                text,
                highlighter,
                state,
            },
        }
    }
}

impl<'a, 'b> Iterator for HighlightIterator<'a, 'b> {
    type Item = (Style, &'b str);

    fn next(&mut self) -> Option<(Style, &'b str)> {
        self.ranged_iterator.next().map(|e| (e.0, e.1))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoredStyle {
    pub foreground: (MatchPower, Color),
    pub background: (MatchPower, Color),
    pub font_style: (MatchPower, FontStyle),
}

#[inline]
fn update_scored<T: Clone>(scored: &mut (MatchPower, T), update: &Option<T>, score: MatchPower) {
    if score > scored.0 {
        if let Some(u) = update {
            scored.0 = score;
            scored.1 = u.clone();
        }
    }
}

impl ScoredStyle {
    fn apply(&mut self, other: &StyleModifier, score: MatchPower) {
        update_scored(&mut self.foreground, &other.foreground, score);
        update_scored(&mut self.background, &other.background, score);
        update_scored(&mut self.font_style, &other.font_style, score);
    }

    fn to_style(&self) -> Style {
        Style {
            foreground: self.foreground.1,
            background: self.background.1,
            font_style: self.font_style.1,
        }
    }

    fn from_style(style: Style) -> ScoredStyle {
        ScoredStyle {
            foreground: (MatchPower(-1.0), style.foreground),
            background: (MatchPower(-1.0), style.background),
            font_style: (MatchPower(-1.0), style.font_style),
        }
    }
}

impl<'a> Highlighter<'a> {
    pub fn new(theme: &'a Theme) -> Highlighter<'a> {
        let mut single_selectors = Vec::new();
        let mut multi_selectors = Vec::new();
        for item in &theme.scopes {
            for sel in &item.scope.selectors {
                if let Some(scope) = sel.extract_single_scope() {
                    single_selectors.push((scope, item.style));
                } else {
                    multi_selectors.push((sel.clone(), item.style));
                }
            }
        }
        // So that deeper matching selectors get checked first
        single_selectors.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        Highlighter {
            theme,
            single_selectors,
            multi_selectors,
        }
    }

    pub fn get_default(&self) -> Style {
        Style {
            foreground: self.theme.settings.foreground.unwrap_or(Color::BLACK),
            background: self.theme.settings.background.unwrap_or(Color::WHITE),
            font_style: FontStyle::empty(),
        }
    }

    fn update_single_cache_for_push(&self, cur: &ScoredStyle, path: &[Scope]) -> ScoredStyle {
        let mut new_style = cur.clone();

        let last_scope = path[path.len() - 1];
        for &(scope, ref modif) in self
            .single_selectors
            .iter()
            .filter(|a| a.0.is_prefix_of(last_scope))
        {
            let single_score = f64::from(scope.len())
                * f64::from(ATOM_LEN_BITS * ((path.len() - 1) as u16)).exp2();
            new_style.apply(modif, MatchPower(single_score));
        }

        new_style
    }

    fn finalize_style_with_multis(&self, cur: &ScoredStyle, path: &[Scope]) -> Style {
        let mut new_style = cur.clone();

        let mult_iter = self
            .multi_selectors
            .iter()
            .filter_map(|&(ref sel, ref style)| sel.does_match(path).map(|score| (score, style)));
        for (score, ref modif) in mult_iter {
            new_style.apply(modif, score);
        }

        new_style.to_style()
    }

    pub fn style_for_stack(&self, stack: &[Scope]) -> Style {
        let mut single_cache = ScoredStyle::from_style(self.get_default());
        for i in 0..stack.len() {
            single_cache = self.update_single_cache_for_push(&single_cache, &stack[0..i + 1]);
        }
        self.finalize_style_with_multis(&single_cache, stack)
    }

    pub fn style_mod_for_stack(&self, path: &[Scope]) -> StyleModifier {
        let mut matching_items: Vec<(MatchPower, &ThemeItem)> = self
            .theme
            .scopes
            .iter()
            .filter_map(|item| item.scope.does_match(path).map(|score| (score, item)))
            .collect();
        matching_items.sort_by_key(|&(score, _)| score);
        let sorted = matching_items.iter().map(|(_, item)| item);

        let mut modifier = StyleModifier {
            background: None,
            foreground: None,
            font_style: None,
        };
        for item in sorted {
            modifier = modifier.apply(item.style);
        }
        modifier
    }
}
