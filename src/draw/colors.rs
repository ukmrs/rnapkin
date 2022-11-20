#[allow(dead_code)]
pub mod default_pallette {
    use plotters::style::RGBColor;
    // gruvbox colors https://github.com/morhetz/gruvbox
    // dark
    pub const DARK_C: RGBColor = RGBColor(184, 187, 38); // green for cytosine
    pub const DARK_U: RGBColor = RGBColor(131, 165, 146); // blue for uracil
    pub const DARK_G: RGBColor = RGBColor(251, 73, 52); // red for guanine
    pub const DARK_A: RGBColor = RGBColor(250, 189, 47); // yellow for adenine
    pub const DARK_X: RGBColor = RGBColor(211, 134, 155); // purple for the unknown
    pub const DARK_BG: RGBColor = RGBColor(40, 40, 40); // background
    pub const DARK_FG: RGBColor = RGBColor(235, 219, 178); // lettering

    // bright ones
    pub const BRIGHT_C: RGBColor = RGBColor(152, 151, 26);
    pub const BRIGHT_U: RGBColor = RGBColor(69, 133, 136);
    pub const BRIGHT_G: RGBColor = RGBColor(204, 36, 29);
    pub const BRIGHT_A: RGBColor = RGBColor(215, 153, 33);
    pub const BRIGHT_X: RGBColor = RGBColor(211, 134, 155);
    pub const BRIGHT_BG: RGBColor = RGBColor(251, 241, 199);
    pub const BRIGHT_FG: RGBColor = RGBColor(60, 56, 54);
}
