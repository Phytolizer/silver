use crate::analysis::text::text_span::TextSpan;

use super::{
    expression_syntax::ExpressionSyntax, syntax_kind::SyntaxKind, syntax_node::SyntaxNodeExt,
    syntax_token::SyntaxToken,
};

pub struct CompilationUnitSyntax {
    expression: ExpressionSyntax,
    end_of_file_token: SyntaxToken,
}

impl CompilationUnitSyntax {
    pub fn new(expression: ExpressionSyntax, end_of_file_token: SyntaxToken) -> Self {
        Self {
            expression,
            end_of_file_token,
        }
    }

    pub fn expression(&self) -> &ExpressionSyntax {
        &self.expression
    }

    pub fn end_of_file_token(&self) -> &SyntaxToken {
        &self.end_of_file_token
    }
}

impl SyntaxNodeExt for CompilationUnitSyntax {
    fn kind(&self) -> super::syntax_kind::SyntaxKind {
        SyntaxKind::CompilationUnit
    }

    fn children(&self) -> Vec<&dyn SyntaxNodeExt> {
        vec![&self.expression, &self.end_of_file_token]
    }

    fn value(&self) -> Option<&crate::analysis::silver_value::SilverValue> {
        None
    }

    fn text(&self) -> Option<&str> {
        None
    }

    fn span(&self) -> TextSpan {
        self.expression.span().start..self.end_of_file_token.span().end
    }
}
