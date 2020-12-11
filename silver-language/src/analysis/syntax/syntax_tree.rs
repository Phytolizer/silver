use std::io::{self, Write};

use super::{
    expression_syntax::ExpressionSyntax, parser::Parser, syntax_node::SyntaxNodeExt,
    syntax_token::SyntaxToken,
};

pub struct SyntaxTree<'source> {
    root: ExpressionSyntax<'source>,
    end_of_file_token: SyntaxToken<'source>,
}

impl<'source> SyntaxTree<'source> {
    pub(crate) fn new(
        root: ExpressionSyntax<'source>,
        end_of_file_token: SyntaxToken<'source>,
    ) -> Self {
        Self {
            root,
            end_of_file_token,
        }
    }

    pub fn parse(text: &'source str) -> Self {
        Parser::parse(text)
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
