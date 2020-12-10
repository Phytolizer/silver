use std::io::{self, BufRead, BufReader, Write};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use view_options::ViewOptions;

mod view_options;

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    let mut reader = BufReader::new(io::stdin());
    let mut line = String::new();
    let mut view_options = ViewOptions::default();

    loop {
        write!(stdout, "silver ")?;
        stdout.execute(SetForegroundColor(Color::Green))?;
        write!(stdout, "➤")?;
        stdout.execute(ResetColor)?;
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
        writeln!(stdout, "Evaluating '{}'", line)?;
    }
    Ok(())
}
