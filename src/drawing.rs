use crate::{bgtask, cli, exit_with_msg, path_from_input};

use image::{open, ImageBuffer, Rgba};
use imageproc::{drawing::draw_antialiased_line_segment_mut as draw_line, pixelops::interpolate};
use std::{env, error::Error, f32::consts::PI};

type ImageRef<'a> = &'a mut ImageBuffer<Rgba<u8>, Vec<u8>>;

const ORIGINAL_IMAGE: &str = "image\\wallpaper.png";
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const CLOCK_CENTER: (i32, i32) = (631, 88);
const LINE_WEIGHT: u8 = 3;

pub fn generate_image(args: &cli::CLI) -> Result<(), Box<dyn Error>> {
    let target = path_from_input(&args.target);
    let image_path = env::current_exe()?
        .ancestors()
        .nth(3)
        .unwrap()
        .join(ORIGINAL_IMAGE);

    let mut image = open(&image_path)?;
    let image = image.as_mut_rgba8().unwrap();

    let (hours, minutes) = args.get_time();
    draw_hand(image, minutes, false);
    draw_hand(image, hours, true);

    let target_str = target.as_path().to_str().unwrap();
    match image.save(&target) {
        Ok(()) => println!("Successfully saved at => {target_str}"),
        Err(e) => exit_with_msg!("{e} (Path: {target_str})", 1),
    }

    if args.set_wallpaper {
        bgtask::try_set_wallpaper(target_str);
    }

    Ok(())
}

fn draw_hand(image: ImageRef, mut hand_value: f32, is_hour_hand: bool) {
    let mut length_scale = 1.;
    if is_hour_hand {
        hand_value *= 5.;
        length_scale = 0.65;
    }

    let length = get_length(hand_value) * length_scale;
    let angle = get_angle(hand_value);
    let (sin, cos) = sin_cos_deg(angle);

    let end = (
        CLOCK_CENTER.0 + (length * cos) as i32,
        CLOCK_CENTER.1 + (length * sin) as i32,
    );

    // repeat to add weight/thickness to the lines
    for _ in 0..LINE_WEIGHT {
        draw_line(image, CLOCK_CENTER, end, WHITE, interpolate);
    }
}

// i don't actually know why this works, but it does, so yeah
// sine wave go brrrr
fn get_length(x: f32) -> f32 {
    5. * (0.21 * x + 7.7).sin() + 32.
}

// https://desmos.com/calculator/ee92c50qh5
fn get_angle(x: f32) -> f32 {
    if x <= 15. {
        16. / 3. * x - 90.
    } else if x <= 30. {
        20. / 3. * x - 110.
    } else if x <= 45. {
        16. / 3. * x - 70.
    } else {
        20. / 3. * x - 130.
    }
}

fn sin_cos_deg(x: f32) -> (f32, f32) {
    (x * PI / 180.).sin_cos()
}
