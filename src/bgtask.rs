use crate::exit_with_msg;
use wallpaper;
use chrono::{DateTime, Local, Timelike};
use std::{
    thread,
    path::PathBuf,
    time::Duration as StdDuration
};

const SIXTY_SECONDS: StdDuration = StdDuration::from_secs(60u64);

pub struct Loop { directory: PathBuf }

impl Loop {
    pub fn in_directory(path: PathBuf) -> Result<Self, String> {
        if !path.is_dir() { return Err(format!("Not a directory ({})", path.display())) }

        Ok(Loop { directory: path })
    }

    pub fn run(self) -> ! {
        let rn = self.set_wallpaper_current_time();

        let delay = 60 - rn.second() as u64;
        thread::sleep(StdDuration::from_secs(delay));

        loop {
            self.set_wallpaper_current_time();
            thread::sleep(SIXTY_SECONDS)
        }
    }

    fn set_wallpaper_current_time(&self) -> DateTime<Local> {
        let rn = Local::now();
        let path = self.directory.join(format!("{}-{}.png", rn.hour(), rn.minute()));

        match wallpaper::set_from_path(path.as_path().to_str().unwrap()) {
            Ok(()) => println!("Wallpaper successfully set ({})", path.display()),
            Err(e) => exit_with_msg(format!("Failed setting wallpaper: {e}\nExiting..."), 1),
        }

        rn
    }
}
