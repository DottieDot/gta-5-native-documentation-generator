use std::{
  fs::{create_dir_all, read_to_string, File},
  io::Write,
  path::PathBuf
};

use clap::{Parser, ValueHint};
use peg::{error::ParseError, str::LineCol};

use crate::parser::sch_parser;

use self::parser::model::Declaration;

mod parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Glob pattern for sch files
  #[arg(short, long, value_hint = ValueHint::FilePath)]
  sch_files: String,

  /// Output directory
  #[arg(short, long, value_hint = ValueHint::DirPath)]
  output: String
}

type ProcessResult = (String, Result<Vec<Declaration>, ParseError<LineCol>>);

fn process_file_contents(path: &PathBuf, contents: String) -> ProcessResult {
  (
    path
      .file_name()
      .and_then(|n| n.to_str())
      .map(|n| n.to_owned())
      .unwrap(),
    sch_parser::sch(&contents)
  )
}

fn process_files(pattern: String) -> anyhow::Result<Vec<ProcessResult>> {
  let result = glob::glob(&pattern)?
    .filter_map(|entry| {
      match &entry {
        Ok(path) if path.is_file() => {
          if let Ok(contents) = read_to_string(path) {
            Some(process_file_contents(&path, contents))
          } else {
            None
          }
        }
        _ => None
      }
    })
    .collect::<Vec<_>>();

  Ok(result)
}

fn write_output_to_file(
  output_dir: &str,
  script: &str,
  decls: Vec<Declaration>
) -> anyhow::Result<()> {
  let path = format!("{output_dir}/{script}.rs");

  println!("Writing to {path}");

  let mut file = File::create(path)?;

  file.write_all(format!("{decls:#?}").as_bytes())?;

  Ok(())
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  create_dir_all(&args.output)?;

  for result in process_files(args.sch_files)? {
    match result {
      (name, Ok(decls)) => {
        println!("Parsed {name}");
        write_output_to_file(&args.output, &name, decls)?;
        println!("Saved output for {name}");
      }
      (name, Err(e)) => {
        println!("Failed to parse {name}:\r\n{e}")
      }
    }
  }

  Ok(())
}
