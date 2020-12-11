use super::{
    silver_value::SilverValue,
    syntax::{
        expression_syntax::ExpressionSyntax, syntax_kind::SyntaxKind, syntax_token::SyntaxToken,
        syntax_tree::SyntaxTree,
    },
};

pub struct Evaluator<'source> {
    syntax_tree: SyntaxTree<'source>,
}

impl<'source> Evaluator<'source> {
    pub fn new(syntax_tree: SyntaxTree<'source>) -> Self {
        Self { syntax_tree }
    }

    pub fn evaluate(&self) -> Option<SilverValue> {
        self.evaluate_expression(self.syntax_tree.root())
    }

    fn evaluate_expression(&self, root: &ExpressionSyntax) -> Option<SilverValue> {
        match root {
            ExpressionSyntax::Literal { literal_token } => literal_token.value().cloned(),
            ExpressionSyntax::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary_expression(left, operator, right),
            ExpressionSyntax::Unary { operator, operand } => {
                self.evaluate_unary_expression(operator, operand)
            }
        }
    }

    fn evaluate_binary_expression(
        &self,
        left: &ExpressionSyntax,
        operator: &SyntaxToken,
        right: &ExpressionSyntax,
    ) -> Option<SilverValue> {
        let left = self.evaluate_expression(left);
        let right = self.evaluate_expression(right);

        match operator.kind() {
            SyntaxKind::PlusToken => Some(SilverValue::Integer(
                left.unwrap().as_integer().unwrap() + right.unwrap().as_integer().unwrap(),
            )),
            SyntaxKind::MinusToken => Some(SilverValue::Integer(
                left.unwrap().as_integer().unwrap() + right.unwrap().as_integer().unwrap(),
            )),
            SyntaxKind::StarToken => Some(SilverValue::Integer(
                left.unwrap().as_integer().unwrap() * right.unwrap().as_integer().unwrap(),
            )),
            SyntaxKind::SlashToken => Some(SilverValue::Integer(
                left.unwrap().as_integer().unwrap() / right.unwrap().as_integer().unwrap(),
            )),
            _ => panic!("unexpected binary operator {}", operator.kind()),
        }
    }

    fn evaluate_unary_expression(
        &self,
        operator: &SyntaxToken,
        operand: &ExpressionSyntax,
    ) -> Option<SilverValue> {
        let operand = self.evaluate_expression(operand);
        match operator.kind() {
            SyntaxKind::PlusToken => operand,
            SyntaxKind::MinusToken => Some(SilverValue::Integer(
                -operand.unwrap().as_integer().unwrap(),
            )),
            _ => panic!("unexpected unary operator {}", operator.kind()),
        }
    }
}
