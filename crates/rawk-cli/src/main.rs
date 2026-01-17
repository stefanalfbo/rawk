use std::{io, path};

use clap::{CommandFactory, Parser};
use rawk_core::awk;

#[derive(Parser, Debug)]
struct Args {
    /// Program text is read from file instead of command line
    #[arg(short = 'f', long = "file", value_name = "program-file")]
    program_file: Option<path::PathBuf>,

    /// Positional arguments: PROGRAM INPUT or INPUT when using -f
    #[arg(value_name = "ARGS", num_args = 0..=2)]
    args: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let (script, input) = if let Some(program_file) = args.program_file {
        let script = std::fs::read_to_string(program_file)?;
        match args.args.as_slice() {
            [input] => (script, input.clone()),
            _ => {
                let mut cmd = Args::command();
                cmd.print_help()?;
                println!();
                return Ok(());
            }
        }
    } else {
        match args.args.as_slice() {
            [script, input] => (script.clone(), input.clone()),
            _ => {
                let mut cmd = Args::command();
                cmd.print_help()?;
                println!();
                return Ok(());
            }
        }
    };

    execute(&script, path::Path::new(&input));

    Ok(())
}

fn execute(script: &str, path: &path::Path) {
    let input_lines = std::fs::read_to_string(path)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let output_lines = awk::execute(script, input_lines);

    for line in output_lines {
        println!("{}", line);
    }
}
