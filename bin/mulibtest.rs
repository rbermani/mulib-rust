#![allow(dead_code)]

extern crate mulib;

mod cli_handlers;
mod repl_funcs;
mod error;

use cli_handlers::repl_shell;
use env_logger::Env;
use structopt::StructOpt;
use error::Result;

#[derive(Debug, Clone, PartialEq, StructOpt)]
#[structopt(name = "mode")]
enum Mode {
    #[structopt(name = "shell")]
    Shell,
}

#[derive(Debug, Clone, StructOpt)]
#[structopt(
    name = "mulib",
    about = "A REPL for testing the mulib rust library"
)]
struct CliOpts {
    #[structopt(subcommand)]
    mode: Option<Mode>,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default()).init();

    let cli_opt = CliOpts::from_args();

    match cli_opt.mode {
        Some(Mode::Shell) => {
            repl_shell()?;
        }
        None => {
            println!("No command mode provided.")
        }
    }
    Ok(())
}
