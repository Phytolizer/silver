use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Write},
};

use crossterm::{
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use silver_language::analysis::{
    compilation::Compilation,
    errors::{error_reporter::ErrorReporter, string_error_reporter::StringErrorReporter},
    silver_value::SilverValue,
    syntax::syntax_tree::SyntaxTree,
    variable_symbol::VariableSymbol,
};
use view_options::ViewOptions;

mod view_options;

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    let mut reader = BufReader::new(io::stdin());
    let mut input = String::new();
    let mut view_options = ViewOptions::default();
    let mut error_reporter = StringErrorReporter::new();
    let mut variables = HashMap::<VariableSymbol, SilverValue>::new();
    let mut text_builder = String::new();

    loop {
        error_reporter.clear();

        stdout.execute(SetForegroundColor(Color::Yellow))?;
        stdout.execute(SetAttribute(Attribute::Bold))?;
        write!(stdout, "silver ")?;
        stdout.execute(SetForegroundColor(Color::Green))?;
        if text_builder.is_empty() {
            write!(stdout, "➤")?;
        } else {
            write!(stdout, "|")?;
        }
        stdout.execute(ResetColor)?;
        stdout.execute(SetAttribute(Attribute::Reset))?;
        write!(stdout, " ")?;
        stdout.flush()?;

        input.clear();
        if reader.read_line(&mut input)? == 0 {
            break;
        }
        let is_blank = input.chars().all(|c| c.is_whitespace()) || input.is_empty();
        if is_blank && text_builder.is_empty() {
            break;
        }
        // check for meta-commands
        if text_builder.is_empty() {
            match input.trim() {
                "#help" => {
                    writeln!(stdout, " -- HELP --")?;
                    writeln!(stdout, "#showTree : Show/hide parse trees")?;
                    writeln!(stdout, "#cls      : Clear the console")?;
                    continue;
                }
                "#showTree" => {
                    view_options.show_tree = !view_options.show_tree;
                    writeln!(
                        stdout,
                        "{}",
                        if view_options.show_tree {
                            "Showing parse trees."
                        } else {
                            "Not showing parse trees."
                        }
                    )?;
                    continue;
                }
                "#cls" => {
                    stdout.execute(Clear(ClearType::All))?;
                    continue;
                }
                _ => {}
            }
        }

        text_builder += &input;
        // evaluate the line
        let parse_tree = SyntaxTree::parse_str(&text_builder, &mut error_reporter);
        if !is_blank && error_reporter.had_error() {
            error_reporter.clear();
            continue;
        }
        if view_options.show_tree {
            parse_tree.pretty_print(&mut stdout)?;
        }
        let mut compilation = Compilation::new(&parse_tree, &mut error_reporter);
        let value = compilation.evaluate(&mut variables);
        if error_reporter.had_error() {
            for error in error_reporter.errors() {
                let line_index = parse_tree.text().get_line_index(error.span().start);
                let line_number = line_index + 1;
                let line = &parse_tree.text().lines()[line_index];
                let character = error.span().start - line.start() + 1;
                stdout.execute(SetForegroundColor(Color::Red))?;
                writeln!(stdout)?;
                writeln!(
                    stdout,
                    "({}, {}) ERROR: {}",
                    line_number,
                    character,
                    error.message()
                )?;

                let prefix = &text_builder[line.start()..error.span().start];
                let highlight = &text_builder[error.span()];
                let suffix = &text_builder[error.span().end..line.end()];

                stdout.execute(ResetColor)?;
                write!(stdout, "    {}", prefix)?;
                stdout.execute(SetForegroundColor(Color::Red))?;
                write!(stdout, "{}", highlight)?;
                stdout.execute(ResetColor)?;
                write!(stdout, "{}", suffix)?;
                writeln!(stdout)?;
            }
        } else {
            stdout.execute(SetForegroundColor(Color::Magenta))?;
            writeln!(stdout, "{}", value.unwrap())?;
            stdout.execute(ResetColor)?;
        }
        text_builder.clear();
    }
    Ok(())
}
