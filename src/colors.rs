use owo_colors::DynColors;
use rand::prelude::SliceRandom;

/// Returns colors sorted in the rainbow order (ROYGBIV). This is a constant
/// function, and is guaranteed to always return the same value.
///
/// # Returns
///
/// Colors sorted in the rainbow order.
pub fn colors() -> [DynColors; 7] {
    [
        "#B80A41", // Red
        "#E23838", // Orange
        "#DAAC06", // Yellow
        "#6EB122", // Green
        "#00938A", // Blue
        "#4E4BA8", // Indigo
        "#BA55D3", // Purple
    ]
    .map(|color| color.parse().unwrap())
}

/// Returns a random color from the `COLORS` array.
///
/// # Returns
///
/// A random color from the `COLORS` array.
pub fn random() -> DynColors {
    let mut rng = rand::thread_rng();
    *colors().choose(&mut rng).unwrap()
}
