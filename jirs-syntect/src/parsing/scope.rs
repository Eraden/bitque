// see DESIGN.md
use std::cmp::{min, Ordering};
use std::collections::HashMap;
use std::fmt;
use std::mem;
use std::str::FromStr;
use std::sync::Mutex;
use std::u16;
use std::u64;

pub const ATOM_LEN_BITS: u16 = 3;

lazy_static! {
    pub static ref SCOPE_REPO: Mutex<ScopeRepository> = Mutex::new(ScopeRepository::new());
}

#[derive(serde::Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Default, Hash, Debug)]
pub struct Scope {
    pub a: u64,
    pub b: u64,
}

// impl Scope {
//     pub fn new(a: u64, b: u64) -> Self {
//         Self { a, b }
//     }
// }

#[derive(Debug)]
pub enum ParseScopeError {
    TooLong,
    TooManyAtoms,
}

pub struct ScopeRepository {
    pub atoms: Vec<String>,
    pub atom_index_map: HashMap<String, usize>,
}

impl std::fmt::Debug for ScopeRepository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ScopeRepository {")?;
        f.write_str(
            format!(
                "  atoms: vec!{:?},\n",
                self.atoms
                    .iter()
                    .map(|s| format!("{:#?}.to_string()", s))
                    .collect::<Vec<String>>()
            )
            .as_str(),
        )?;
        f.write_str(
            format!("  atom_index_map: {{\n    let mut v = std::collections::HashMap::new();\n")
                .as_str(),
        )?;
        for (n, v) in self.atom_index_map.iter() {
            f.write_str(
                format!("    v.insert({:?}.to_string(), {:?}.to_string());\n", n, v).as_str(),
            )?;
        }
        f.write_str(format!("    v\n  }}\n").as_str())?;
        f.write_str("}")
    }
}

/// A stack/sequence of scopes. This is used both to represent hierarchies for a given
/// token of text, as well as in `ScopeSelectors`. Press `ctrl+shift+p` in Sublime Text
/// to see the scope stack at a given point.
/// Also see [the TextMate docs](https://manual.macromates.com/en/scope_selectors).
///
/// Example for a JS string inside a script tag in a Rails `ERB` file:
/// `text.html.ruby text.html.basic source.js.embedded.html string.quoted.double.js`
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ScopeStack {
    pub clear_stack: Vec<Vec<Scope>>,
    pub scopes: Vec<Scope>,
}

#[derive(serde::Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum ClearAmount {
    TopN(usize),
    All,
}

impl std::fmt::Debug for ClearAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClearAmount::TopN(n) => f.write_str(format!("ClearAmount::TopN({})", n).as_str()),
            ClearAmount::All => f.write_str("ClearAmount::All"),
        }
    }
}

/// A change to a scope stack. Generally `Noop` is only used internally and you don't have
/// to worry about ever getting one back from a public function.
/// Use `ScopeStack#apply` to apply this change.
#[derive(Clone, PartialEq, Eq)]
pub enum ScopeStackOp {
    Push(Scope),
    Pop(usize),
    /// used for the clear_scopes feature
    Clear(ClearAmount),
    /// restores cleared scopes
    Restore,
    Noop,
}

impl std::fmt::Debug for ScopeStackOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScopeStackOp::Push(n) => f.write_str(format!("ScopeStackOp::Push({:#?})", n).as_str()),
            ScopeStackOp::Pop(n) => f.write_str(format!("ScopeStackOp::Pop({})", n).as_str()),
            ScopeStackOp::Clear(n) => {
                f.write_str(format!("ScopeStackOp::Clear({:#?})", n).as_str())
            }
            ScopeStackOp::Restore => f.write_str("ScopeStackOp::Restore"),
            ScopeStackOp::Noop => f.write_str("ScopeStackOp::Noop"),
        }
    }
}

/// Used for `ScopeStack::apply_with_hook`
#[derive(Clone, PartialEq, Eq)]
pub enum BasicScopeStackOp {
    Push(Scope),
    Pop,
}

impl std::fmt::Debug for BasicScopeStackOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicScopeStackOp::Push(n) => {
                f.write_str(format!("BasicScopeStackOp::Push({:#?})", n).as_str())
            }
            BasicScopeStackOp::Pop => f.write_str("BasicScopeStackOp::Pop"),
        }
    }
}

fn pack_as_u16s(atoms: &[usize]) -> Result<Scope, ParseScopeError> {
    let mut res = Scope { a: 0, b: 0 };

    for (i, &n) in atoms.iter().enumerate() {
        if n >= (u16::MAX as usize) - 2 {
            return Err(ParseScopeError::TooManyAtoms);
        }
        let small = (n + 1) as u64; // +1 since we reserve 0 for unused

        if i < 4 {
            let shift = (3 - i) * 16;
            res.a |= small << shift;
        } else {
            let shift = (7 - i) * 16;
            res.b |= small << shift;
        }
    }
    Ok(res)
}

impl ScopeRepository {
    fn new() -> ScopeRepository {
        ScopeRepository {
            atoms: Vec::new(),
            atom_index_map: HashMap::new(),
        }
    }

    pub fn build(&mut self, s: &str) -> Result<Scope, ParseScopeError> {
        if s.is_empty() {
            return Ok(Scope { a: 0, b: 0 });
        }
        let parts: Vec<usize> = s
            .trim_end_matches('.')
            .split('.')
            .map(|a| self.atom_to_index(a))
            .collect();
        if parts.len() > 8 {
            return Err(ParseScopeError::TooManyAtoms);
        }
        pack_as_u16s(&parts[..])
    }

    pub fn to_string(&self, scope: Scope) -> String {
        let mut s = String::new();
        for i in 0..8 {
            let atom_number = scope.atom_at(i);
            // println!("atom {} of {:x}-{:x} = {:x}",
            //     i, scope.a, scope.b, atom_number);
            if atom_number == 0 {
                break;
            }
            if i != 0 {
                s.push_str(".");
            }
            s.push_str(self.atom_str(atom_number));
        }
        s
    }

    fn atom_to_index(&mut self, atom: &str) -> usize {
        if let Some(index) = self.atom_index_map.get(atom) {
            return *index;
        }

        self.atoms.push(atom.to_owned());
        let index = self.atoms.len() - 1;
        self.atom_index_map.insert(atom.to_owned(), index);

        index
    }

    /// Return the string for an atom number returned by `Scope#atom_at`
    pub fn atom_str(&self, atom_number: u16) -> &str {
        &self.atoms[(atom_number - 1) as usize]
    }
}

impl Scope {
    /// Parses a `Scope` from a series of atoms separated by
    /// `.` characters. Example: `Scope::new("meta.rails.controller")`
    pub fn new(s: &str) -> Result<Scope, ParseScopeError> {
        let mut repo = SCOPE_REPO.lock().unwrap();
        repo.build(s.trim())
    }

    /// Gets the atom number at a given index.
    /// I can't think of any reason you'd find this useful.
    /// It is used internally for turning a scope back into a string.
    pub fn atom_at(self, index: usize) -> u16 {
        let shifted = if index < 4 {
            self.a >> ((3 - index) * 16)
        } else if index < 8 {
            self.b >> ((7 - index) * 16)
        } else {
            panic!("atom index out of bounds {:?}", index);
        };
        (shifted & 0xFFFF) as u16
    }

    #[inline]
    fn missing_atoms(self) -> u32 {
        let trail = if self.b == 0 {
            self.a.trailing_zeros() + 64
        } else {
            self.b.trailing_zeros()
        };
        trail / 16
    }

    /// return the number of atoms in the scope
    #[inline(always)]
    pub fn len(self) -> u32 {
        8 - self.missing_atoms()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// returns a string representation of this scope, this requires locking a
    /// global repo and shouldn't be done frequently.
    pub fn build_string(self) -> String {
        let repo = SCOPE_REPO.lock().unwrap();
        repo.to_string(self)
    }

    pub fn is_prefix_of(self, s: Scope) -> bool {
        let mask: (u64, u64) = match self.missing_atoms() {
            8 => (0, 0),
            4 => (u64::MAX, 0),
            v if v > 4 => (u64::MAX << ((v - 4) * 16), 0),
            v => (u64::MAX, u64::MAX << (v * 16)),
        };

        // xor to find the difference
        let ax = (self.a ^ s.a) & mask.0;
        let bx = (self.b ^ s.b) & mask.1;
        ax == 0 && bx == 0
    }
}

impl FromStr for Scope {
    type Err = ParseScopeError;

    fn from_str(s: &str) -> Result<Scope, ParseScopeError> {
        Scope::new(s)
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.build_string();
        write!(f, "{}", s)
    }
}

/// Wrapper to get around the fact Rust f64 doesn't implement Ord
/// and there is no non-NaN float type
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct MatchPower(pub f64);
impl Eq for MatchPower {}
impl Ord for MatchPower {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl ScopeStack {
    pub fn new() -> ScopeStack {
        Self {
            clear_stack: Vec::new(),
            scopes: Vec::new(),
        }
    }

    pub fn new_with_args(clear_stack: Vec<Vec<Scope>>, scopes: Vec<Scope>) -> Self {
        Self {
            clear_stack,
            scopes,
        }
    }

    /// Note: creating a ScopeStack with this doesn't contain information
    /// on what to do when clear_scopes contexts end.
    pub fn from_vec(v: Vec<Scope>) -> ScopeStack {
        ScopeStack {
            clear_stack: Vec::new(),
            scopes: v,
        }
    }

    #[inline]
    pub fn push(&mut self, s: Scope) {
        self.scopes.push(s);
    }

    #[inline]
    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    /// Modifies this stack according to the operation given
    /// use this to create a stack from a `Vec` of changes
    /// given by the parser.
    pub fn apply(&mut self, op: &ScopeStackOp) {
        self.apply_with_hook(op, |_, _| {})
    }

    /// Modifies this stack according to the operation given and calls the hook for each basic operation.
    /// Like `apply` but calls `hook` for every basic modification (as defined by `BasicScopeStackOp`).
    /// Use this to do things only when the scope stack changes.
    #[inline]
    pub fn apply_with_hook<F>(&mut self, op: &ScopeStackOp, mut hook: F)
    where
        F: FnMut(BasicScopeStackOp, &[Scope]),
    {
        match *op {
            ScopeStackOp::Push(scope) => {
                self.scopes.push(scope);
                hook(BasicScopeStackOp::Push(scope), self.as_slice());
            }
            ScopeStackOp::Pop(count) => {
                for _ in 0..count {
                    self.scopes.pop();
                    hook(BasicScopeStackOp::Pop, self.as_slice());
                }
            }
            ScopeStackOp::Clear(amount) => {
                let cleared = match amount {
                    ClearAmount::TopN(n) => {
                        // don't try to clear more scopes than are on the stack
                        let to_leave = self.scopes.len() - min(n, self.scopes.len());
                        self.scopes.split_off(to_leave)
                    }
                    ClearAmount::All => {
                        let mut cleared = Vec::new();
                        mem::swap(&mut cleared, &mut self.scopes);
                        cleared
                    }
                };
                let clear_amount = cleared.len();
                self.clear_stack.push(cleared);
                for _ in 0..clear_amount {
                    hook(BasicScopeStackOp::Pop, self.as_slice());
                }
            }
            ScopeStackOp::Restore => match self.clear_stack.pop() {
                Some(ref mut to_push) => {
                    for s in to_push {
                        self.scopes.push(*s);
                        hook(BasicScopeStackOp::Push(*s), self.as_slice());
                    }
                }
                None => panic!("tried to restore cleared scopes, but none were cleared"),
            },
            ScopeStackOp::Noop => (),
        }
    }

    /// Prints out each scope in the stack separated by spaces
    /// and then a newline. Top of the stack at the end.
    pub fn debug_print(&self, repo: &ScopeRepository) {
        for s in &self.scopes {
            print!("{} ", repo.to_string(*s));
        }
        println!();
    }

    /// Return the bottom n elements of the stack.
    /// Equivalent to &scopes[0..n] on a Vec
    pub fn bottom_n(&self, n: usize) -> &[Scope] {
        &self.scopes[0..n]
    }

    /// Return a slice of the scopes in this stack
    #[inline]
    pub fn as_slice(&self) -> &[Scope] {
        &self.scopes[..]
    }

    /// Return the height/length of this stack
    #[inline]
    pub fn len(&self) -> usize {
        self.scopes.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn does_match(&self, stack: &[Scope]) -> Option<MatchPower> {
        let mut sel_index: usize = 0;
        let mut score: f64 = 0.0;
        for (i, scope) in stack.iter().enumerate() {
            let sel_scope = self.scopes[sel_index];
            if sel_scope.is_prefix_of(*scope) {
                let len = sel_scope.len();
                // equivalent to score |= len << (ATOM_LEN_BITS*i) on a large unsigned
                score += f64::from(len) * f64::from(ATOM_LEN_BITS * (i as u16)).exp2();
                sel_index += 1;
                if sel_index >= self.scopes.len() {
                    return Some(MatchPower(score));
                }
            }
        }
        None
    }
}

impl FromStr for ScopeStack {
    type Err = ParseScopeError;

    /// Parses a scope stack from a whitespace separated list of scopes.
    fn from_str(s: &str) -> Result<ScopeStack, ParseScopeError> {
        let mut scopes = Vec::new();
        for name in s.split_whitespace() {
            scopes.push(Scope::from_str(name)?)
        }
        Ok(ScopeStack::from_vec(scopes))
    }
}

impl fmt::Display for ScopeStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for s in &self.scopes {
            write!(f, "{} ", s)?;
        }
        Ok(())
    }
}
