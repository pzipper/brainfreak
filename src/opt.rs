//! The optimizer for Brainfreak.

use crate::{inst::Inst, parser::Token};

/// A context for optimizing Brainfreak code.
#[derive(Clone, Debug)]
pub struct Optimizer {
    /// The instructions in the optimizer.
    insts: Vec<Inst>,
}

impl Optimizer {
    /// Creates a new [Optimizer].
    #[inline]
    pub fn new() -> Self {
        Self { insts: Vec::new() }
    }

    /// Writes the provided [Token]s as [Inst]s to this [Optimizer].
    #[inline]
    pub fn write_tokens(&mut self, tokens: &[Token]) {
        self.insts.extend(tokens.iter().map(Inst::from_token))
    }

    /// Returns the optimized instructions.
    #[inline]
    pub fn finish(self) -> Vec<Inst> {
        self.insts
    }
}
