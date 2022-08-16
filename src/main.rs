use clap::{builder::Arg, crate_authors, crate_version, Command};
use owo_colors::{DynColors, OwoColorize};
use rand::seq::SliceRandom;

static DOG1: &str = include_str!("./art/dog1.txt");

fn main() {
    let colors: [DynColors; 7] = [
        "#B80A41", // Red
        "#E23838", // Orange
        "#DAAC06", // Yellow
        "#6EB122", // Green
        "#00938A", // Blue
        "#4E4BA8", // Indigo
        "#BA55D3", // Purple
    ]
    .map(|color| color.parse().unwrap());

    let command = Command::new("clidogs")
        .about("∪･ω･U")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(Arg::new("rainbow").long("rainbow"))
        .arg(Arg::new("random").long("random").conflicts_with("rainbow"));

    let lines: Vec<_> = DOG1.lines().collect();

    let matches = command.get_matches();

    if matches.contains_id("rainbow") {
        lines
            .iter()
            .zip(colors.iter().cycle())
            .map(|(line, color)| line.color(*color))
            .for_each(|line| println!("{}", line));
    } else if matches.contains_id("random") {
        let mut rng = rand::thread_rng();
        for line in lines {
            println!("{}", line.color(*colors.choose(&mut rng).unwrap()));
        }
    } else {
        lines.iter().for_each(|line| println!("{}", line));
    }
}
