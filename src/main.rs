use clap::{crate_authors, crate_version, Command};

static DOG1: &str = include_str!("./art/dog1.txt");

fn main() {
    let command = Command::new("clidogs")
        .about("∪･ω･U")
        .version(crate_version!())
        .author(crate_authors!("\n"));

    let matches = command.get_matches();

    if !matches.args_present() {
        println!("{}", DOG1);
    }
}
