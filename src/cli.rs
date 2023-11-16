use chrono::{Local, Timelike};
use clap::Parser;

#[derive(Parser)]
pub struct CLI {
    #[arg(
        long,
        short = 't',
        help = "Destination to save output\nIf the --loop flag is used, this will instead specify a loop directory",
        required = true
    )]
    pub target: String,

    #[arg(
        long,
        short = 'M',
        help = "Where the minute hand should be",
        default_value = "current"
    )]
    pub minutes: String,

    #[arg(
        long,
        short = 'H',
        help = "Where the hour hand should be",
        default_value = "current"
    )]
    pub hours: String,

    #[arg(
        long,
        help = "Sets the output as the desktop wallpaper",
        default_value = "false"
    )]
    pub wallpaper: bool,

    #[arg(
        long = "loop",
        help = "Change the wallpaper every minute, loop through a directory with pre-generated images"
    )]
    pub loop_dir: bool,

    #[arg(
        long = "run-once",
        help = "Set the wallpaper to the current time, using a pre-generated image from a target directory"
    )]
    pub run_once: bool
}

impl CLI {
    pub fn get_time(&self) -> (f32, f32) {
        let rn = Local::now();
        let minutes = self.minutes.parse::<f32>().unwrap_or(rn.minute() as f32) % 60.;
        let mut hours = self.hours.parse::<f32>().unwrap_or(rn.hour() as f32) % 12.;

        // moves the hour hand with the minutes so it's not stuck in one spot the whole time
        hours += minutes / 60.;

        (hours, minutes)
    }
}
