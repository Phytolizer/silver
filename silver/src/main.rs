use std::io::{self, BufRead, BufReader, Write};

use crossterm::{
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use silver_language::analysis::{
    compilation::Compilation,
    errors::{error_reporter::ErrorReporter, string_error_reporter::StringErrorReporter},
    syntax::syntax_tree::SyntaxTree,
};
use view_options::ViewOptions;

mod view_options;

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    let mut reader = BufReader::new(io::stdin());
    let mut line = String::new();
    let mut view_options = ViewOptions::default();
    let mut error_reporter = StringErrorReporter::new();

    loop {
        error_reporter.clear();

        stdout.execute(SetForegroundColor(Color::Yellow))?;
        stdout.execute(SetAttribute(Attribute::Bold))?;
        write!(stdout, "silver ")?;
        stdout.execute(SetForegroundColor(Color::Green))?;
        write!(stdout, "➤")?;
        stdout.execute(ResetColor)?;
        stdout.execute(SetAttribute(Attribute::Reset))?;
        write!(stdout, " ")?;
        stdout.flush()?;

        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }

        // check for meta-commands
        match line.trim() {
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

        // evaluate the line
        stdout.execute(SetForegroundColor(Color::Blue))?;
        write!(stdout, "Executing")?;
        stdout.execute(ResetColor)?;
        writeln!(stdout, " '{}'", line.trim())?;
        let parse_tree = SyntaxTree::parse(line.trim(), &mut error_reporter);
        if view_options.show_tree {
            parse_tree.pretty_print(&mut stdout)?;
        }
        let mut compilation = Compilation::new(parse_tree, &mut error_reporter);
        let value = compilation.evaluate();
        if error_reporter.had_error() {
            for error in error_reporter.errors() {
                stdout.execute(SetForegroundColor(Color::Red))?;
                writeln!(stdout, "ERROR: {}", error.message())?;

                let prefix = &line[..error.span().start];
                let highlight = &line[error.span()];
                let suffix = &line[error.span().end..];

                stdout.execute(ResetColor)?;
                write!(stdout, "    {}", prefix)?;
                stdout.execute(SetForegroundColor(Color::Red))?;
                write!(stdout, "{}", highlight)?;
                stdout.execute(ResetColor)?;
                write!(stdout, "{}", suffix)?;
            }
        } else {
            writeln!(stdout, "{}", value.unwrap())?;
        }
    }
    Ok(())
}
