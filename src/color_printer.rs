use owo_colors::OwoColorize;

use crate::colors;

/// Colors the given lines in rainbow colors.
///
/// # Arguments
///
/// * `lines` - The lines to color.
///
/// # Returns
///
/// The colored output.
pub fn rainbow(string: &str) -> String {
    string
        .lines()
        .zip(colors::colors().iter().cycle())
        .map(|(line, color)| line.color(*color).to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Colors the given lines in random colors.
///
/// # Arguments
///
/// * `lines` - The lines to color.
///
/// # Returns
///
/// The colored output.
pub fn random(string: &str) -> String {
    string
        .lines()
        .map(|line| {
            let color = colors::random();
            line.color(color).to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}
