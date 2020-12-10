use std::iter::Peekable;

use crate::analysis::silver_value::SilverValue;

use super::{syntax_kind::SyntaxKind, syntax_token::SyntaxToken};

pub struct Lexer;

impl<'source> Lexer {
    pub fn get_tokens(text: &'source str) -> Vec<SyntaxToken<'source>> {
        let mut tokens = vec![];
        let mut iterator = text.char_indices().peekable();

        while let Some(token) = Self::next_token(text, &mut iterator) {
            let kind = token.kind();
            tokens.push(token);
            if kind == SyntaxKind::EndOfFile {
                break;
            }
        }

        tokens
    }

    fn next_token(
        text: &'source str,
        iterator: &mut Peekable<impl Iterator<Item = (usize, char)>>,
    ) -> Option<SyntaxToken<'source>> {
        match iterator.peek() {
            Some((_, c)) if c.is_numeric() => Self::read_number_token(text, iterator),
            Some((_, c)) if c.is_whitespace() => Self::read_whitespace_token(text, iterator),
            Some(&(pos, '+')) => {
                iterator.next();
                Self::fixed_token(pos, SyntaxKind::Plus, "+")
            }
            Some(&(pos, '-')) => {
                iterator.next();
                Self::fixed_token(pos, SyntaxKind::Minus, "-")
            }
            Some(&(pos, '*')) => {
                iterator.next();
                Self::fixed_token(pos, SyntaxKind::Star, "*")
            }
            Some(&(pos, '/')) => {
                iterator.next();
                Self::fixed_token(pos, SyntaxKind::Slash, "/")
            }
            Some(&(pos, '(')) => {
                iterator.next();
                Self::fixed_token(pos, SyntaxKind::OpenParenthesis, "(")
            }
            Some(&(pos, ')')) => {
                iterator.next();
                Self::fixed_token(pos, SyntaxKind::CloseParenthesis, ")")
            }
            Some(&(pos, _)) => {
                iterator.next();
                Self::fixed_token(pos, SyntaxKind::Bad, "")
            }
            None => Some(SyntaxToken::new(
                SyntaxKind::EndOfFile,
                text.len(),
                "",
                None,
            )),
        }
    }

    /// Create a token with a known lexeme.
    fn fixed_token(
        pos: usize,
        kind: SyntaxKind,
        text: &'static str,
    ) -> Option<SyntaxToken<'source>> {
        Some(SyntaxToken::new(kind, pos, text, None))
    }

    fn read_number_token(
        text: &'source str,
        iterator: &mut Peekable<impl Iterator<Item = (usize, char)>>,
    ) -> Option<SyntaxToken<'source>> {
        let (start, _) = iterator.next().unwrap();
        while let Some((_, c)) = iterator.peek() {
            if !c.is_numeric() {
                break;
            }
            iterator.next();
        }
        let (position, _) = iterator
            .peek()
            .cloned()
            .unwrap_or_else(|| (text.len(), '\0'));
        let text = &text[start..position];
        let parsed = match text.parse() {
            Ok(p) => p,
            Err(_) => return None,
        };
        let value = Some(SilverValue::Integer(parsed));
        Some(SyntaxToken::new(SyntaxKind::Number, start, text, value))
    }

    fn read_whitespace_token(
        text: &'source str,
        iterator: &mut Peekable<impl Iterator<Item = (usize, char)>>,
    ) -> Option<SyntaxToken<'source>> {
        let (start, _) = iterator.next().unwrap();
        while let Some((_, c)) = iterator.peek() {
            if !c.is_whitespace() {
                break;
            }
            iterator.next();
        }
        let (position, _) = iterator
            .peek()
            .cloned()
            .unwrap_or_else(|| (text.len(), '\0'));
        let text = &text[start..position];
        Some(SyntaxToken::new(SyntaxKind::Whitespace, start, text, None))
    }
}
