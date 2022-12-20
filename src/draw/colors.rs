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

    // TRUST me it absolutely KILLS ME that HIGHLIGHT_1's index is 0, but alas 'HIGHLIGHT_0' means no highlight
    // theres no sane alternative though; this is the way unfortunately;
    // other solutions are either dumb or are unintuitive for the user
    /// An array of 9 "flavored, distinct but not too avant-garde" colors:
    /// ```text
    /// - CHERRY_RED:          (221, 37, 30)   #DD2520
    /// - BLUEBERRY_BLUE:      (54, 111, 177)  #3670B1
    /// - LIME_GREEN:          (128, 255, 0)   #80FF00
    /// - GRAPE_PURPLE:        (106, 47, 107)  #6A2F6B
    /// - LEMON_YELLOW:        (255, 253, 56)  #FFFD38
    /// - ORANGE_CREAM:        (255, 84, 3)    #FF5403
    /// - PEANUT_BUTTER_BROWN: (164, 134, 69)  #A48644
    /// - POMEGRANATE_PINK:    (217, 87, 122)  #D95770
    /// - MINT_GREEN:          (141, 215, 145) #8DD791
    /// ```
    pub const HIGHLIGHTS: [RGBColor; 9] = [
        RGBColor(221, 37, 30),   // CHERRY_RED
        RGBColor(54, 111, 177),  // BLUEBERRY_BLUE
        RGBColor(128, 255, 0),   // LIME_GREEN
        RGBColor(106, 47, 107),  // GRAPE_PURPLE
        RGBColor(255, 253, 56),  // LEMON_YELLOW
        RGBColor(255, 84, 3),    // ORANGE_CREAM
        RGBColor(164, 134, 69),  // PEANUT_BUTTER_BROWN
        RGBColor(217, 87, 122),  // POMEGRANATE_PINK
        RGBColor(141, 215, 145), // MINT_GREEN
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
    pub bg: RGBColor,
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
