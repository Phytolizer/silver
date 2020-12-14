use std::collections::HashMap;

use super::{
    binding::binder::Binder, errors::error_reporter::ErrorReporter, evaluator::Evaluator,
    silver_value::SilverValue, syntax::syntax_tree::SyntaxTree, variable_symbol::VariableSymbol,
};

pub struct Compilation<'syntax, 'reporter> {
    syntax: &'syntax SyntaxTree,
    error_reporter: &'reporter mut dyn ErrorReporter,
}

impl<'syntax, 'reporter> Compilation<'syntax, 'reporter> {
    pub fn new(
        syntax: &'syntax SyntaxTree,
        error_reporter: &'reporter mut dyn ErrorReporter,
    ) -> Self {
        Self {
            syntax,
            error_reporter,
        }
    }

    pub fn evaluate(
        &mut self,
        variables: &mut HashMap<VariableSymbol, SilverValue>,
    ) -> Option<SilverValue> {
        let mut binder = Binder::new(variables, self.error_reporter);
        let bound_tree = binder.bind(self.syntax.root());
        if self.error_reporter.had_error() {
            return None;
        }
        let mut evaluator = Evaluator::new(variables);
        Some(evaluator.evaluate(&bound_tree))
    }
}
