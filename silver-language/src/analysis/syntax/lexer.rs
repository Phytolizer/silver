use std::{collections::VecDeque, iter::Peekable};

use crate::analysis::silver_type::SilverType;
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
                if iterator.peek().map(|&(_, c)| c == '=').unwrap_or(false) {
                    iterator.next();
                    return Self::fixed_token(pos, SyntaxKind::BangEqualsToken, "!=");
                } else {
                    return Self::fixed_token(pos, SyntaxKind::BangToken, "!");
                }
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
            Some(&(pos, '=')) => {
                iterator.next();
                if iterator.peek().map(|&(_, c)| c == '=').unwrap_or(false) {
                    iterator.next();
                    return Self::fixed_token(pos, SyntaxKind::EqualsEqualsToken, "==");
                } else {
                    return Self::fixed_token(pos, SyntaxKind::EqualsToken, "=");
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

        if iterator.peek().map(|&(i, _)| i).unwrap_or(0) == start_pos {
            iterator.next();
        }
        error_reporter.report_invalid_character(start_pos..start_pos + 1, start_c);
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
                error_reporter.report_invalid_number(start..position, text, SilverType::Integer);
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

#[cfg(test)]
mod tests {
    use credibility::{aver, aver_eq, test_block, TestBlock, TestReporter};
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    use crate::analysis::errors::string_error_reporter::StringErrorReporter;

    use super::syntax_facts::SyntaxKindWithText;
    use super::*;

    fn get_all_valid_tokens() -> Vec<(&'static str, SyntaxKind)> {
        let static_tokens = SyntaxKind::iter()
            .filter_map(|k| {
                if let Some(t) = k.get_text() {
                    Some((t, k))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let dynamic_tokens = vec![
            ("a", SyntaxKind::IdentifierToken),
            ("abc", SyntaxKind::IdentifierToken),
            ("abcABC", SyntaxKind::IdentifierToken),
            ("1", SyntaxKind::NumberToken),
            ("123", SyntaxKind::NumberToken),
        ];
        static_tokens
            .iter()
            .cloned()
            .chain(dynamic_tokens.iter().cloned())
            .collect()
    }

    fn get_all_separator_tokens() -> Vec<(&'static str, SyntaxKind)> {
        vec![
            (" ", SyntaxKind::WhitespaceToken),
            ("  ", SyntaxKind::WhitespaceToken),
            ("\r", SyntaxKind::WhitespaceToken),
            ("\n", SyntaxKind::WhitespaceToken),
            ("\r\n", SyntaxKind::WhitespaceToken),
        ]
    }

    fn lexer_lexes_token<T: TestReporter>(tb: &mut TestBlock<T>, input: &str, kind: SyntaxKind) {
        let mut error_reporter = StringErrorReporter::new();
        let tokens = Lexer::get_tokens(input, &mut error_reporter);
        aver!(tb, !error_reporter.had_error());
        aver_eq!(tb, 2, tokens.len());
        aver_eq!(tb, kind, tokens[0].kind());
        aver_eq!(tb, input, tokens[0].text());
    }

    #[test]
    fn lexes_single_token() {
        test_block!(tb, "Lexing single tokens", {
            for (input, kind) in get_all_valid_tokens() {
                lexer_lexes_token(&mut tb, input, kind);
            }
            Ok(())
        });
    }

    fn token_pair_requires_separator(t1kind: SyntaxKind, t2kind: SyntaxKind) -> bool {
        let t1_is_keyword = t1kind.to_string().ends_with("Keyword");
        let t2_is_keyword = t2kind.to_string().ends_with("Keyword");

        t1_is_keyword && t2_is_keyword
            || t1_is_keyword && t2kind == SyntaxKind::IdentifierToken
            || t1kind == SyntaxKind::IdentifierToken && t2_is_keyword
            || t1kind == SyntaxKind::IdentifierToken && t2kind == SyntaxKind::IdentifierToken
            || t1kind == SyntaxKind::BangToken && t2kind == SyntaxKind::EqualsEqualsToken
            || t1kind == SyntaxKind::BangToken && t2kind == SyntaxKind::EqualsToken
            || t1kind == SyntaxKind::EqualsToken && t2kind == SyntaxKind::EqualsEqualsToken
            || t1kind == SyntaxKind::EqualsToken && t2kind == SyntaxKind::EqualsToken
            || t1kind == SyntaxKind::NumberToken && t2kind == SyntaxKind::NumberToken
    }

    fn lexer_lexes_token_pair<T: TestReporter>(
        tb: &mut TestBlock<T>,
        t1text: &str,
        t1kind: SyntaxKind,
        t2text: &str,
        t2kind: SyntaxKind,
    ) {
        let mut error_reporter = StringErrorReporter::new();
        let input = format!("{}{}", t1text, t2text);
        let tokens = Lexer::get_tokens(&input, &mut error_reporter);
        for error in error_reporter.errors() {
            println!("{}", error.message());
        }
        aver!(tb, !error_reporter.had_error());
        aver_eq!(tb, 3, tokens.len());
        aver_eq!(tb, t1kind, tokens[0].kind());
        aver_eq!(tb, t2kind, tokens[1].kind());
        aver_eq!(
            tb,
            input,
            format!("{}{}", tokens[0].text(), tokens[1].text())
        );
    }

    fn lexer_lexes_token_pair_with_separator<T: TestReporter>(
        tb: &mut TestBlock<T>,
        t1text: &str,
        t1kind: SyntaxKind,
        separator_text: &str,
        separator_kind: SyntaxKind,
        t2text: &str,
        t2kind: SyntaxKind,
    ) {
        let mut error_reporter = StringErrorReporter::new();
        let input = format!("{}{}{}", t1text, separator_text, t2text);
        let tokens = Lexer::get_tokens(&input, &mut error_reporter);
        for error in error_reporter.errors() {
            println!("{}", error.message());
        }
        aver!(tb, !error_reporter.had_error());
        aver_eq!(tb, 4, tokens.len());
        aver_eq!(tb, t1kind, tokens[0].kind());
        aver_eq!(tb, separator_kind, tokens[1].kind());
        aver_eq!(tb, t2kind, tokens[2].kind());
        aver_eq!(
            tb,
            input,
            format!(
                "{}{}{}",
                tokens[0].text(),
                tokens[1].text(),
                tokens[2].text()
            )
        );
    }

    #[test]
    fn lexes_token_pairs() {
        test_block!(tb, "Lexing token pairs", {
            for (t1text, t1kind) in get_all_valid_tokens() {
                for (t2text, t2kind) in get_all_valid_tokens() {
                    if token_pair_requires_separator(t1kind, t2kind) {
                        for (separator_text, separator_kind) in get_all_separator_tokens() {
                            lexer_lexes_token_pair_with_separator(
                                &mut tb,
                                t1text,
                                t1kind,
                                separator_text,
                                separator_kind,
                                t2text,
                                t2kind,
                            );
                        }
                    } else {
                        lexer_lexes_token_pair(&mut tb, t1text, t1kind, t2text, t2kind);
                    }
                }
            }
            Ok(())
        });
    }
}
