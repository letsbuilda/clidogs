use clap::{crate_authors, crate_version, Command};
use rand::seq::SliceRandom;
use owo_colors::{DynColors, OwoColorize};

static DOG1: &str = include_str!("./art/dog1.txt");

fn main() {
    let command = Command::new("clidogs")
        .about("∪･ω･U")
        .version(crate_version!())
        .author(crate_authors!("\n"));

    let matches = command.get_matches();

    if !matches.args_present() {
        let colors: [DynColors; 6] = [
            "#B80A41", "#4E4BA8", "#6EB122", "#DAAC06", "#00938A", "#E23838",
        ].map(|color| color.parse().unwrap());

        let mut rng = rand::thread_rng();

        for line in DOG1.lines() {
            let color = colors.choose(&mut rng).unwrap();
            println!("{}", line.color(*color));
        }
    }
}
