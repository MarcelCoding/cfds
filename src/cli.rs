use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub(crate) struct Cli {
  #[arg(short, long)]
  pub(crate) token: String,
  #[arg(short, long)]
  pub(crate) dry_run: bool,
  pub(crate) zone_files: Vec<PathBuf>,
}

