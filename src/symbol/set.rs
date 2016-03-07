//! Informs whether symbols are terminal or nonterminal.

use std::iter;

use bit_vec;
use bit_vec::BitVec;

use grammar::{ContextFree, ContextFreeRef};
use rule::GrammarRule;
use symbol::Symbol;

/// A set of symbols in the form of a bit vector.
pub struct SymbolBitSet {
    bit_vec: BitVec,
}

/// An iterator over a symbol set.
pub struct Iter<'a> {
    iter: iter::Enumerate<bit_vec::Iter<'a>>
}

impl SymbolBitSet {
    /// Constructs a `SymbolBitSet`.
    pub fn new<'a, G>(grammar: &'a G, elem: bool) -> Self
        where G: ContextFree,
              &'a G: ContextFreeRef<'a, Target = G>,
    {
        SymbolBitSet {
            bit_vec: BitVec::from_elem(grammar.num_syms(), elem),
        }
    }

    /// Gathers information about whether symbols are terminal or nonterminal.
    /// Constructs a set of terminal symbols.
    ///
    /// Constructs a data structure in O(n) time.
    pub fn terminal_set<'a, G>(grammar: &'a G) -> Self
        where G: ContextFree,
              &'a G: ContextFreeRef<'a, Target = G>,
    {
        let mut set = SymbolBitSet::new(grammar, true);
        for rule in grammar.rules() {
            if !rule.rhs().is_empty() {
                set.bit_vec.set(rule.lhs().into(), false);
            }
        }
        set
    }

    /// Checks whether a given symbol is in this set.
    pub fn has_sym(&self, sym: Symbol) -> bool {
        self.bit_vec[sym.into()]
    }

    /// Converts into a bit vector.
    pub fn into_bit_vec(self) -> BitVec {
        self.bit_vec
    }

    /// Iterates over symbols in the set.
    pub fn iter(&self) -> Iter {
        Iter { iter: self.bit_vec.iter().enumerate() }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Symbol;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((id, is_present)) = self.iter.next() {
            if is_present {
                return Some(Symbol::from(id));
            }
        }
        None
    }
}