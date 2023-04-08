mod color_printer;
mod colors;
mod command;

/// The first dog's ASCII art.
static DOG1: &str = include_str!("./art/dog1.txt");

fn main() {
    let command = command::create_command();

    println!(
        "{}",
        match command::get_color_mode(command) {
            command::ColorMode::None => DOG1.to_string(),
            command::ColorMode::Rainbow => color_printer::rainbow(DOG1),
            command::ColorMode::Random => color_printer::random(DOG1),
        }
    );
}
