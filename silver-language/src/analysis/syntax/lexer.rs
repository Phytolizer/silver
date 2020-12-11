use std::{collections::VecDeque, iter::Peekable};

use crate::analysis::{errors::error_reporter::ErrorReporter, silver_value::SilverValue};

use super::{syntax_facts, syntax_kind::SyntaxKind, syntax_token::SyntaxToken};

pub struct Lexer;

impl<'source> Lexer {
    pub fn get_tokens(
        text: &'source str,
        error_reporter: &mut dyn ErrorReporter,
    ) -> VecDeque<SyntaxToken<'source>> {
        let mut tokens = VecDeque::new();
        let mut iterator = text.char_indices().peekable();

        while let Some(token) = Self::next_token(text, &mut iterator, error_reporter) {
            let kind = token.kind();
            tokens.push_back(token);
            if kind == SyntaxKind::EndOfFileToken {
                break;
            }
        }

        tokens
    }

    fn next_token(
        text: &'source str,
        iterator: &mut Peekable<impl Iterator<Item = (usize, char)>>,
        error_reporter: &mut dyn ErrorReporter,
    ) -> Option<SyntaxToken<'source>> {
        let (start_pos, start_c) = iterator.peek().cloned().unwrap_or((0, '\0'));
        match iterator.peek() {
            Some((_, c)) if c.is_numeric() => {
                return Self::read_number_token(text, iterator, error_reporter);
            }
            Some((_, c)) if c.is_whitespace() => {
                return Self::read_whitespace_token(text, iterator);
            }
            Some((_, c)) if c.is_alphabetic() => {
                return Self::read_identifier_or_keyword_token(text, iterator);
            }
            Some(&(pos, '+')) => {
                iterator.next();
                return Self::fixed_token(pos, SyntaxKind::PlusToken, "+");
            }
            Some(&(pos, '-')) => {
                iterator.next();
                return Self::fixed_token(pos, SyntaxKind::MinusToken, "-");
            }
            Some(&(pos, '*')) => {
                iterator.next();
                return Self::fixed_token(pos, SyntaxKind::StarToken, "*");
            }
            Some(&(pos, '/')) => {
                iterator.next();
                return Self::fixed_token(pos, SyntaxKind::SlashToken, "/");
            }
            Some(&(pos, '(')) => {
                iterator.next();
                return Self::fixed_token(pos, SyntaxKind::OpenParenthesisToken, "(");
            }
            Some(&(pos, ')')) => {
                iterator.next();
                return Self::fixed_token(pos, SyntaxKind::CloseParenthesisToken, ")");
            }
            Some(&(pos, '!')) => {
                iterator.next();
                return Self::fixed_token(pos, SyntaxKind::BangToken, "!");
            }
            Some(&(pos, '&')) => {
                iterator.next();
                if iterator.peek().map(|&(_, c)| c == '&').unwrap_or(false) {
                    iterator.next();
                    return Self::fixed_token(pos, SyntaxKind::AmpersandAmpersandToken, "&&");
                }
            }
            Some(&(pos, '|')) => {
                iterator.next();
                if iterator.peek().map(|&(_, c)| c == '|').unwrap_or(false) {
                    iterator.next();
                    return Self::fixed_token(pos, SyntaxKind::PipePipeToken, "||");
                }
            }
            None => {
                return Some(SyntaxToken::new(
                    SyntaxKind::EndOfFileToken,
                    text.len(),
                    "",
                    None,
                ))
            }
            _ => {}
        }

        if iterator.peek().unwrap().0 == start_pos {
            iterator.next();
        }
        error_reporter.report_error(format!("Bad character in input: '{}'", start_c));
        Self::fixed_token(start_pos, SyntaxKind::BadToken, "")
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
        error_reporter: &mut dyn ErrorReporter,
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
            Err(_) => {
                error_reporter.report_error(format!("Numeric literal '{}' is invalid", text));
                return None;
            }
        };
        let value = Some(SilverValue::Integer(parsed));
        Some(SyntaxToken::new(
            SyntaxKind::NumberToken,
            start,
            text,
            value,
        ))
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
        Some(SyntaxToken::new(
            SyntaxKind::WhitespaceToken,
            start,
            text,
            None,
        ))
    }

    fn read_identifier_or_keyword_token(
        text: &'source str,
        iterator: &mut Peekable<impl Iterator<Item = (usize, char)>>,
    ) -> Option<SyntaxToken<'source>> {
        let (start, _) = iterator.next().unwrap();
        while let Some((_, c)) = iterator.peek() {
            if !c.is_alphabetic() {
                break;
            }
            iterator.next();
        }
        let (position, _) = iterator
            .peek()
            .cloned()
            .unwrap_or_else(|| (text.len(), '\0'));
        let text = &text[start..position];
        let kind = syntax_facts::keyword_kind(text);
        Some(SyntaxToken::new(kind, start, text, None))
    }
}
