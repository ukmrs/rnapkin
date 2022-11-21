use plotters::style::RGBColor;

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

#[derive(Debug, Clone)]
pub struct ColorTheme {
    /// cytosine
    pub c: RGBColor,
    /// uracil
    pub u: RGBColor,
    /// guanine
    pub g: RGBColor,
    /// adenine
    pub a: RGBColor,
    /// unknown nt
    pub x: RGBColor,
    /// background
    pub bg: RGBColor,
    /// foreground
    pub fg: RGBColor,
}

impl ColorTheme {

    pub fn dark() -> Self {
        Self {
            a: default_pallette::DARK_A,
            c: default_pallette::DARK_C,
            g: default_pallette::DARK_G,
            u: default_pallette::DARK_U,
            x: default_pallette::DARK_X,
            bg: default_pallette::DARK_BG,
            fg: default_pallette::DARK_FG,
        }
    }

    pub fn bright() -> Self {
        Self {
            a: default_pallette::BRIGHT_A,
            c: default_pallette::BRIGHT_C,
            g: default_pallette::BRIGHT_G,
            u: default_pallette::BRIGHT_U,
            x: default_pallette::BRIGHT_X,
            bg: default_pallette::BRIGHT_BG,
            fg: default_pallette::BRIGHT_FG,
        }
    }

    pub fn black() -> Self {
        Self {
            bg: RGBColor(0, 0, 0),
            ..Self::bright()
        }
    }

    pub fn white() -> Self {
        Self {
            bg: RGBColor(255, 255, 255),
            ..Self::bright()
        }
    }

}

impl Default for ColorTheme {
    fn default() -> Self {
        Self::dark()
    }
}
