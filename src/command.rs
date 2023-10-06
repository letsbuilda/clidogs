use clap::{builder::Arg, crate_authors, crate_version, Command};

/// The color mode.
pub enum ColorMode {
    None,
    Rainbow,
    Random,
}

/// Creates the command. This is guaranteed to always return the same value, so
/// this should be called once and the result stored.
///
/// # Returns
///
/// A `clap` `Command`.
pub fn create() -> Command {
    Command::new("clidogs")
        .about("∪･ω･U")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .arg(Arg::new("rainbow").long("rainbow"))
        .arg(Arg::new("random").long("random").conflicts_with("rainbow"))
}

/// Returns the color mode. This assumes that the command passed is the one
/// returned by `create`.
///
/// # Arguments
///
/// * `command` - The command to get the color mode from.
///
/// # Returns
///
/// The color mode.
pub fn get_color_mode(command: Command) -> ColorMode {
    let matches = command.get_matches();

    if matches.contains_id("rainbow") {
        ColorMode::Rainbow
    } else if matches.contains_id("random") {
        ColorMode::Random
    } else {
        ColorMode::None
    }
}
