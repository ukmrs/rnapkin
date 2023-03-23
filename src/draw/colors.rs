use plotters::style::{RGBAColor, RGBColor};

#[allow(dead_code)]
pub mod default_pallette {
    use plotters::style::{RGBAColor, RGBColor};
    // gruvbox colors https://github.com/morhetz/gruvbox
    // dark
    pub const DARK_C: RGBColor = RGBColor(184, 187, 38); // green for cytosine
    pub const DARK_U: RGBColor = RGBColor(131, 165, 146); // blue for uracil
    pub const DARK_G: RGBColor = RGBColor(251, 73, 52); // red for guanine
    pub const DARK_A: RGBColor = RGBColor(250, 189, 47); // yellow for adenine
    pub const DARK_X: RGBColor = RGBColor(211, 134, 155); // purple for the unknown
    pub const DARK_BG: RGBAColor = RGBAColor(40, 40, 40, 1.0); // background
    pub const DARK_FG: RGBColor = RGBColor(235, 219, 178); // lettering

    // bright ones
    pub const BRIGHT_C: RGBColor = RGBColor(152, 151, 26);
    pub const BRIGHT_U: RGBColor = RGBColor(69, 133, 136);
    pub const BRIGHT_G: RGBColor = RGBColor(204, 36, 29);
    pub const BRIGHT_A: RGBColor = RGBColor(215, 153, 33);
    pub const BRIGHT_X: RGBColor = RGBColor(211, 134, 155);
    pub const BRIGHT_BG: RGBAColor = RGBAColor(251, 241, 199, 1.0);
    pub const BRIGHT_FG: RGBColor = RGBColor(60, 56, 54);

    /// An array of 9 colors:
    /// ```text
    /// - RED:                 (255, 3, 3)     #FF0303
    /// - BLUE:                (18, 93, 152)   #125D98
    /// - GREEN:               (55, 146, 55)   #379237
    /// - PINK:                (255, 105, 180) #FF69B4
    /// - ORANGE:              (245, 134, 52)  #F58634
    /// - PURPLE:              (146, 67, 238)  #9243EE
    /// - LEMON:               (240, 212, 58)  #F0D43A
    /// - BROWN:               (158, 117, 64)  #9E7540
    /// - TURQUOISE:           (64, 224, 208)  #40E0D0
    /// ```
    pub const HIGHLIGHTS: [RGBColor; 9] = [
        RGBColor(255, 3, 3),     // RED
        RGBColor(18, 93, 152),   // BLUE
        RGBColor(55, 146, 55),   // GREEN
        RGBColor(255, 105, 180), // PINK
        RGBColor(245, 134, 52),  // ORANGE
        RGBColor(146, 67, 238),  // PURPLE
        RGBColor(240, 212, 58),  // YELLOW
        RGBColor(158, 117, 64),  // BROWN
        RGBColor(64, 224, 208),  // TURQUOISE
    ];
}

// TODO i mean this obviously can be u8 its just more convenient as usize
pub fn user_input_to_highlight_indices(hls: &str) -> Vec<Option<usize>> {
    hls.bytes()
        .map(|c| (c as usize).checked_sub(0x31))
        .collect()
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
    pub bg: RGBAColor,
    /// foreground
    pub fg: RGBColor,
    /// since 0 means no highlight; highlight1 is 0indexed and so on
    pub highlights: [RGBColor; 9],
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
            highlights: default_pallette::HIGHLIGHTS,
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
            highlights: default_pallette::HIGHLIGHTS,
        }
    }

    pub fn black() -> Self {
        Self {
            bg: RGBAColor(0, 0, 0, 1.),
            ..Self::bright()
        }
    }

    pub fn white() -> Self {
        Self {
            bg: RGBAColor(255, 255, 255, 1.),
            ..Self::bright()
        }
    }
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self::dark()
    }
}
