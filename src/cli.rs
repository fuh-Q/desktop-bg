use clap::Parser;

#[derive(Parser)]
pub struct CLI {
    #[arg(
        long,
        short='t',
        help="Destination to save output",
        default_value="image/generated.png",
    )]
    pub target: String,

    #[arg(
        long,
        short='M',
        help="Where the minute hand should be",
        default_value="current"
    )]
    pub minutes: String,

    #[arg(
        long,
        short='H',
        help="Where the hour hand should be",
        default_value="current"
    )]
    pub hours: String,

    #[arg(
        long,
        help="Sets the output as the desktop wallpaper",
        default_value="false",
    )]
    pub wallpaper: bool,
}
