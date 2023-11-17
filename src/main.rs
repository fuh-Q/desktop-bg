mod cli;
mod drawing;
mod bgtask;

use clap::Parser;
use std::{
    env,
    error::Error,
    fmt::Display,
    path::PathBuf,
};

fn path_from_input<S>(input: S) -> PathBuf
where S: AsRef<str>
{
    env::current_dir().unwrap().join(input.as_ref().replace("/", "\\"))
    // windows moment
}

fn exit_with_msg<S>(msg: S, code: i32) -> !
where S: AsRef<str> + Display
{
    if code != 0 { eprintln!("{msg}"); }
    else { println!("{msg}"); }
    std::process::exit(code);
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = cli::CLI::parse();
    if cli_args.loop_dir {
        bgtask::start_loop(path_from_input(&cli_args.target));
    } else if cli_args.run_once {
        bgtask::Loop::in_directory(path_from_input(&cli_args.target))?.set_current_time();
        std::process::exit(0);
    }

    if let Err(e) = drawing::generate_image(&cli_args) {
        exit_with_msg(format!("{e}"), 1);
    }

    Ok(())
}
