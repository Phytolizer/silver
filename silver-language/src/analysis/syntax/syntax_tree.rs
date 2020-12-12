use std::{
    collections::VecDeque,
    io::{self, Write},
};

use crate::analysis::errors::error_reporter::ErrorReporter;

use super::{
    expression_syntax::ExpressionSyntax, lexer::Lexer, parser::Parser, syntax_node::SyntaxNodeExt,
    syntax_token::SyntaxToken,
};

pub struct SyntaxTree<'source> {
    root: ExpressionSyntax<'source>,
    // TODO the end-of-file token will be used for diagnostics
    #[allow(dead_code)]
    end_of_file_token: SyntaxToken<'source>,
}

impl<'source, 'reporter> SyntaxTree<'source> {
    pub(crate) fn new(
        root: ExpressionSyntax<'source>,
        end_of_file_token: SyntaxToken<'source>,
    ) -> Self {
        Self {
            root,
            end_of_file_token,
        }
    }

    pub fn parse(text: &'source str, error_reporter: &'reporter mut dyn ErrorReporter) -> Self {
        Parser::parse(text, error_reporter)
    }

    pub fn parse_tokens(
        text: &'source str,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> VecDeque<SyntaxToken<'source>> {
        Lexer::get_tokens(text, error_reporter)
    }

    pub(crate) fn root(&self) -> &ExpressionSyntax {
        &self.root
    }

    pub fn pretty_print(&self, writer: &mut dyn Write) -> io::Result<()> {
        self.pretty_print_recursive(&self.root, writer, String::new(), true)
    }

    fn pretty_print_recursive(
        &self,
        root: &dyn SyntaxNodeExt,
        writer: &mut dyn Write,
        mut indent: String,
        is_last: bool,
    ) -> io::Result<()> {
        write!(writer, "{}", indent)?;
        write!(writer, "{}", if is_last { "\\--" } else { "+--" })?;
        write!(writer, "{}", root.kind())?;
        if let Some(value) = root.value() {
            write!(writer, " {}", value)?;
        }
        writeln!(writer)?;

        indent += if is_last { "   " } else { "|  " };
        for (i, &child) in root.children().iter().enumerate() {
            self.pretty_print_recursive(
                child,
                writer,
                indent.clone(),
                i == root.children().len() - 1,
            )?;
        }
        Ok(())
    }
}
