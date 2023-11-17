use crate::exit_with_msg;

use ctrlc;
use wallpaper;
use chrono::{Local, Timelike};
use std::{
    thread,
    path::PathBuf,
    time::Duration as StdDuration,
};

pub struct Loop { directory: PathBuf }

impl Loop {
    pub fn in_directory(path: PathBuf) -> Result<Self, String> {
        if !path.is_dir() { return Err(format!("Not a directory ({})", path.display())); }

        Ok(Self { directory: path })
    }

    pub fn run(&self) -> ! {
        loop {
            self.set_current_time();
            let rn = Local::now();

            let delay = 60 - rn.second() as u64;
            thread::sleep(StdDuration::from_secs(delay));
        }
    }

    pub fn set_current_time(&self) {
        let rn = Local::now();
        let path = self.directory.join(format!("{}-{}.png", rn.hour(), rn.minute()));

        try_set_wallpaper(path.as_path().to_str().unwrap());
    }
}

pub fn start_loop(path: PathBuf) -> ! {
    if let Err(e) = ctrlc::set_handler(|| exit_with_msg("Exiting...", 0)) {
        exit_with_msg(format!("Failed setting CTRL-C handler: {e}\n\nExiting..."), 1);
    }

    match Loop::in_directory(path) {
        Ok(task) => task.run(),
        Err(e) => exit_with_msg(e, 1),
    }
}

pub fn try_set_wallpaper(path_str: &str) {
    match wallpaper::set_from_path(path_str) {
        Ok(()) => println!("Wallpaper successfully set ({path_str})"),
        Err(e) => exit_with_msg(format!("Failed setting wallpaper: {e}\nExiting..."), 1),
    }
}
