pub use iced::Color;

// These values were determined using a color picker chrome extension on a picture of an OP-1.
// As such, these are likely imperfect but they are close enough.

/// The color of the blue knob and blue key accents (#59b4ed)
pub const BLUE: Color = Color {
    r: 0.34765625,
    g: 0.703125,
    b: 0.92578125,
    a: 1.0,
};

/// The color of the green knob and green key accents (#00ce72)
pub const GREEN: Color = Color {
    r: 0.0,
    g: 0.8046875,
    b: 0.4453125,
    a: 1.0,
};

/// The color of the white knob (#fbf9fa)
pub const WHITE: Color = Color {
    r: 0.98046875,
    g: 0.97265625,
    b: 0.9765625,
    a: 1.0,
};

/// The color of the orange knob and orange key accents (#fd6924)
pub const ORANGE: Color = Color {
    r: 0.98828125,
    g: 0.41015625,
    b: 0.140625,
    a: 1.0,
};

/// The color of the aluminum body of the OP-1 (#d9dce5)
pub const LIGHT_GRAY: Color = Color {
    r: 0.84765625,
    g: 0.859375,
    b: 0.89453125,
    a: 1.0,
};

/// The color of the text printed on the OP-1's keys (#353238)
pub const DARK_GRAY: Color = Color {
    r: 0.20703125,
    g: 0.1953125,
    b: 0.21875,
    a: 1.0,
};

/// The color of the black circles on the piano's black keys (#151316)
pub const BLACK: Color = Color {
    r: 0.08203125,
    g: 0.07421875,
    b: 0.0859375,
    a: 1.0,
};
