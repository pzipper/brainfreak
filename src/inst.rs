use crate::parser::Token;

/// A Brainfreak instruction.
///
/// Includes some optimized instructions to make the final result smaller/faster.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Inst {
    /// Increments the data that the data pointer currently points to.
    DataInc,

    /// Decrements the data that the data pointer currently points to.
    DataDec,

    /// Increments the data pointer.
    PtrInc,

    /// Decrements the data pointer.
    PtrDec,

    /// Outputs the data that the data pointer currently points to.
    Output,

    /// Inputs a byte of data to the location that the data pointer currently points to.
    Input,

    /// A loop that runs until the byte at the current data pointer is 0.
    Loop(Vec<Inst>),
}

impl Inst {
    /// Creates an [Inst] from the provided token.
    pub fn from_token(token: &Token) -> Self {
        match token {
            Token::DataInc => Self::DataInc,
            Token::DataDec => Self::DataDec,
            Token::PtrInc => Self::PtrInc,
            Token::PtrDec => Self::PtrDec,
            Token::Output => Self::Output,
            Token::Input => Self::Input,
            Token::Loop(tokens) => {
                Self::Loop(tokens.iter().map(Self::from_token).collect::<Vec<_>>())
            }
        }
    }
}
