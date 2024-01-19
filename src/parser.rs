use std::str::Chars;

/// Any Brainfreak token.
#[derive(Debug)]
pub enum Token {
    /// `+`
    DataInc,

    /// `-`
    DataDec,

    /// `>`
    PtrInc,

    /// `<`
    PtrDec,

    /// `.`
    Output,

    /// `,`
    Input,

    /// `[]`
    Loop(Vec<Token>),
}

/// An error which occurred during parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// `[` group was started but no matching `]` was found.
    UnmatchedBrackets {
        /// The index of the first bracket.
        idx: usize,
    },
}

/// A parser for Brainfreak source.
#[derive(Debug)]
pub struct Parser<'a> {
    chars: Chars<'a>,

    /// The current index into the `chars` array, used for debugging.
    idx: usize,
}

impl<'a> Parser<'a> {
    /// Creates a new Brainfreak [Parser] for the provided string.
    pub fn new(str: &'a str) -> Self {
        Self {
            chars: str.chars(),
            idx: 0,
        }
    }

    /// Returns the next character in the parser without iterating it.
    #[inline]
    fn peek_char(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Returns the next character in the [Parser], if any.  Keeps track of the *idx* of the
    /// current character and should be used rather than accessing the *chars* iterator directly.
    fn next_char(&mut self) -> Option<char> {
        let next_char = self.chars.next()?;
        self.idx += next_char.len_utf8();
        Some(next_char)
    }

    /// Parses a `[]` loop.  Assumes the first `[` has been iterated past already.
    fn parse_loop(&mut self) -> Result<Vec<Token>, ParseError> {
        // We are already iterated past the first bracket, so the index needs to be modified to
        // match it.
        let start_idx = self.idx - 1;

        let mut tokens = Vec::new();

        while let Some(next_char) = self.peek_char() {
            if next_char == ']' {
                break;
            }

            tokens.push(match self.parse_token()? {
                Some(token) => token,
                None => continue,
            });
        }

        match self.next_char() {
            Some(']') => {}
            _ => return Err(ParseError::UnmatchedBrackets { idx: start_idx }),
        }

        Ok(tokens)
    }

    /// Parses a single token.
    fn parse_token(&mut self) -> Result<Option<Token>, ParseError> {
        let next_char = match self.next_char() {
            Some(next_char) => next_char,
            None => return Ok(None),
        };

        match next_char {
            '>' => Ok(Some(Token::PtrInc)),
            '<' => Ok(Some(Token::PtrDec)),
            '+' => Ok(Some(Token::DataInc)),
            '-' => Ok(Some(Token::DataDec)),
            '.' => Ok(Some(Token::Output)),
            ',' => Ok(Some(Token::Input)),
            '[' => Ok(Some(Token::Loop(self.parse_loop()?))),
            _ => Ok(None),
        }
    }

    /// Parses all tokens from the provided file.
    pub fn parse(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();

        while let Some(_) = self.peek_char() {
            tokens.push(match self.parse_token()? {
                Some(token) => token,
                None => continue,
            });
        }

        Ok(tokens)
    }
}
