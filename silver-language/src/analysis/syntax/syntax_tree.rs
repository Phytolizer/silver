use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::{
    collections::VecDeque,
    io::{stdout, Write},
    sync::Arc,
};

use crate::analysis::{errors::error_reporter::ErrorReporter, text::source_text::SourceText};

use super::{
    compilation_unit_syntax::CompilationUnitSyntax, lexer::Lexer, parser::Parser,
    syntax_node::SyntaxNodeExt, syntax_token::SyntaxToken,
};

pub struct SyntaxTree {
    root: CompilationUnitSyntax,
    text: Arc<SourceText>,
}

impl<'reporter> SyntaxTree {
    fn new(text: Arc<SourceText>, error_reporter: &'reporter mut dyn ErrorReporter) -> Self {
        let root = Parser::parse_compilation_unit(text.clone(), error_reporter);

        Self { text, root }
    }

    pub fn parse(text: Arc<SourceText>, error_reporter: &'reporter mut dyn ErrorReporter) -> Self {
        Self::new(text, error_reporter)
    }

    pub fn parse_str<S: AsRef<str>>(
        text: S,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> Self {
        Self::parse(
            Arc::new(SourceText::from(text.as_ref().to_string())),
            error_reporter,
        )
    }

    pub fn parse_tokens(
        text: Arc<SourceText>,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> VecDeque<SyntaxToken> {
        Lexer::get_tokens(text, error_reporter)
    }

    pub(crate) fn root(&self) -> &CompilationUnitSyntax {
        &self.root
    }

    pub fn text(&self) -> &SourceText {
        &self.text
    }

    pub fn pretty_print(&self) -> anyhow::Result<()> {
        self.pretty_print_recursive(&self.root, &mut stdout(), true, String::new(), true)
    }

    pub fn pretty_print_to(&self, writer: &mut dyn Write) -> anyhow::Result<()> {
        self.pretty_print_recursive(&self.root, writer, false, String::new(), true)
    }

    fn pretty_print_recursive(
        &self,
        root: &dyn SyntaxNodeExt,
        mut writer: &mut dyn Write,
        writer_is_stdout: bool,
        mut indent: String,
        is_last: bool,
    ) -> anyhow::Result<()> {
        write!(writer, "{}", indent)?;
        if writer_is_stdout {
            writer.execute(SetForegroundColor(Color::Grey))?;
        }
        write!(writer, "{}", if is_last { "\\--" } else { "+--" })?;
        if writer_is_stdout {
            writer.execute(SetForegroundColor(
                if root.kind().to_string().ends_with("Token")
                    || root.kind().to_string().ends_with("Keyword")
                {
                    Color::Blue
                } else {
                    Color::Cyan
                },
            ))?;
        }
        write!(writer, "{}", root.kind())?;
        if let Some(value) = root.value() {
            write!(writer, " {}", value)?;
        }
        writeln!(writer)?;

        if writer_is_stdout {
            writer.execute(ResetColor)?;
        }

        indent += if is_last { "   " } else { "|  " };
        for (i, &child) in root.children().iter().enumerate() {
            self.pretty_print_recursive(
                child,
                writer,
                writer_is_stdout,
                indent.clone(),
                i == root.children().len() - 1,
            )?;
        }
        Ok(())
    }
}
