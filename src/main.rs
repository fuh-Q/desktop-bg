mod bgtask;
mod cli;
mod drawing;

use std::{env, error::Error, path::PathBuf};

use clap::Parser;

#[macro_export]
macro_rules! exit_with_msg {
    ($msg:tt, 0) => {{
        println!($msg);
        std::process::exit(0);
    }};

    ($msg:tt, $code:expr) => {{
        eprintln!($msg);
        std::process::exit($code);
    }};
}

#[rustfmt::skip]
fn path_from_input<S: AsRef<str>>(input: S) -> PathBuf {
    env::current_dir().unwrap().join(
        input.as_ref().replace("/", "\\")
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = cli::CLI::parse();

    if cli_args.loop_dir {
        bgtask::start_loop(path_from_input(&cli_args.target));
    } else if cli_args.run_once {
        let task = bgtask::Loop::in_directory(path_from_input(&cli_args.target))?;
        task.set_current_time(cli_args.wait);
        std::process::exit(0);
    }

    if let Err(e) = drawing::generate_image(&cli_args) {
        exit_with_msg!("{e}", 1);
    }

    Ok(())
}
