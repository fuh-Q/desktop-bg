mod cli;
mod drawing;
mod bgtask;

use ctrlc;
use wallpaper;
use clap::Parser;
use image::{Rgba, open};
use std::{
    env,
    error::Error,
    fmt::Display,
    path::PathBuf,
    io::Result as ioResult,
};

const ORIGINAL_IMAGE: &str = "image\\wallpaper.png";
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);

fn path_from_input<S>(input: S) -> ioResult<PathBuf>
where S: AsRef<str>
{
    Ok(env::current_dir()?.join(input.as_ref().replace("/", "\\")))
    // windows moment
}

fn exit_with_msg<S>(msg: S, code: i32) -> !
where S: AsRef<str> + Display
{
    if code != 0 { eprintln!("{msg}") }
    else { println!("{msg}") }
    std::process::exit(code);
}

fn handle_loop(path: PathBuf) -> ! {
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
    if cli_args.loop_dir { handle_loop(path_from_input(&cli_args.target)?) }

    let image_path = env::current_exe()?.ancestors().nth(3).unwrap().join(ORIGINAL_IMAGE);
    let target = path_from_input(&cli_args.target)?;

    let mut image = open(&image_path)?;
    let image = image.as_mut_rgba8().unwrap();

    let (hours, minutes) = cli_args.get_time();

    drawing::draw_hand(image, minutes, false, WHITE);
    drawing::draw_hand(image, hours, true, WHITE);

    let target_str = target.as_path().to_str().unwrap();
    match image.save(&target) {
        Ok(()) => println!("Successfully saved at => {target_str}"),
        Err(e) => exit_with_msg(format!("{e} (Path: {target_str})"), 1)
    }

    if cli_args.wallpaper {
        match wallpaper::set_from_path(target_str) {
            Ok(()) => println!("Wallpaper successfully set ({target_str})"),
            Err(e) => eprintln!("Failed setting wallpaper: {e}")
        }
    }

    Ok(())
}
