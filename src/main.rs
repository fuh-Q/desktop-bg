mod cli;

use wallpaper;
use clap::Parser;
use image::{ImageBuffer, Rgba, open};
use std::{f32::consts::PI, error::Error, env};
use chrono::{prelude::{DateTime, Local}, Timelike};
use imageproc::{
    pixelops::interpolate,
    drawing::draw_antialiased_line_segment_mut as draw_line,
};

type ImageRef<'a> = &'a mut ImageBuffer<Rgba<u8>, Vec<u8>>;
const ORIGINAL_IMAGE: &str = "image\\wallpaper.png";

const CLOCK_CENTER: (i32, i32) = (631, 88);
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const LINE_WEIGHT: u8 = 3;

// i don't actually know why this works, but it does, so yeah
// sine wave go brrrr
fn get_length(x: f32) -> f32 { 5. * (0.21 * x + 7.7).sin() + 32. }

// https://desmos.com/calculator/ee92c50qh5
fn get_angle(x: f32) -> f32 {
    if x <= 15. { 16./3. * x - 90. } // 16/3x - 90
    else if x <= 30. { 20./3. * x - 110. } // 20/3x - 110
    else if x <= 45. { 16./3. * x - 70. } // 16/3x - 70
    else { 20./3. * x - 130. } // 20/3x - 130
}

fn sin_cos_deg(x: f32) -> (f32, f32) { (x*PI/180.).sin_cos() }

fn get_time(cli_args: &cli::CLI) -> (f32, f32) {
    let rn: DateTime<Local> = Local::now();
    let minutes = cli_args.minutes.parse::<f32>().unwrap_or(rn.minute() as f32) % 60.;
    let mut hours = cli_args.hours.parse::<f32>().unwrap_or(rn.hour() as f32) % 12.;

    // moves the hour hand with the minutes so it's not stuck in one spot the whole time
    hours += minutes / 60.;

    (hours, minutes)
}

fn draw_hand(image: ImageRef, mut hand_value: f32, is_hour_hand: bool, color: Rgba<u8>) {
    let mut length_scale = 1.;
    if is_hour_hand {
        hand_value *= 5.;
        length_scale = 0.65;
    }

    let length = get_length(hand_value) * length_scale;
    let angle = get_angle(hand_value);
    let (sin, cos) = sin_cos_deg(angle);

    let end = (CLOCK_CENTER.0 + (length * cos) as i32, CLOCK_CENTER.1 + (length * sin) as i32);

    // repeat to add weight/thickness to the lines
    for _ in 0..LINE_WEIGHT { draw_line(image, CLOCK_CENTER, end, color, interpolate) }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = cli::CLI::parse();
    let image_path = env::current_exe()?.ancestors().nth(3).unwrap().join(ORIGINAL_IMAGE);
    let target = env::current_dir()?.join(&cli_args.target);
    let target_str = target.as_os_str().to_str().unwrap();
    if !target.exists() {
        eprintln!("Path {target_str} not found\n\nTry specifying one with the --target flag");
        std::process::exit(1);
    }

    let mut image = open(&image_path)?;
    let image = image.as_mut_rgba8().unwrap();

    let (hours, minutes) = get_time(&cli_args);

    draw_hand(image, minutes, false, WHITE);
    draw_hand(image, hours, true, WHITE);

    image.save(&target)?;
    if cli_args.wallpaper {
        wallpaper::set_from_path(target_str)?;
    }

    Ok(())
}
