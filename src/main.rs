mod cli;
mod drawing;
mod bgtask;

use ctrlc;
use image::open;
use clap::Parser;
use std::{
    env,
    error::Error,
    fmt::Display,
    path::PathBuf,
};

const ORIGINAL_IMAGE: &str = "image\\wallpaper.png";

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

fn start_loop(path: PathBuf) -> ! {
    if let Err(e) = ctrlc::set_handler(|| exit_with_msg("Exiting...", 0)) {
        exit_with_msg(format!("Failed setting CTRL-C handler: {e}\n\nExiting..."), 1);
    }

    match bgtask::Loop::in_directory(path) {
        Ok(task) => task.run(),
        Err(e) => exit_with_msg(e, 1),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = cli::CLI::parse();
    if cli_args.loop_dir {
        start_loop(path_from_input(&cli_args.target));
    } else if cli_args.run_once {
        bgtask::Loop::in_directory(path_from_input(&cli_args.target))?.set_current_time();
        std::process::exit(0);
    }

    let image_path = env::current_exe()?.ancestors().nth(3).unwrap().join(ORIGINAL_IMAGE);
    let target = path_from_input(&cli_args.target);

    let mut image = open(&image_path)?;
    let image = image.as_mut_rgba8().unwrap();

    let (hours, minutes) = cli_args.get_time();

    drawing::draw_hand(image, minutes, false);
    drawing::draw_hand(image, hours, true);

    let target_str = target.as_path().to_str().unwrap();
    match image.save(&target) {
        Ok(()) => println!("Successfully saved at => {target_str}"),
        Err(e) => exit_with_msg(format!("{e} (Path: {target_str})"), 1),
    }

    if cli_args.wallpaper {
        bgtask::try_set_wallpaper(target_str);
    }

    Ok(())
}
