use macroquad::color::Color;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemeMode {
    CatppuccinMocha,
    TokyoNight,
    NordSerenity,
    RosePine,
    Cyberpunk,
    SunsetZen,
}

impl ThemeMode {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeMode::CatppuccinMocha => "Catppuccin Mocha",
            ThemeMode::TokyoNight => "Tokyo Night",
            ThemeMode::NordSerenity => "Nord Serenity",
            ThemeMode::RosePine => "Rosé Pine",
            ThemeMode::Cyberpunk => "Cyberpunk",
            ThemeMode::SunsetZen => "Sunset Zen",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ThemeMode::CatppuccinMocha => ThemeMode::TokyoNight,
            ThemeMode::TokyoNight => ThemeMode::NordSerenity,
            ThemeMode::NordSerenity => ThemeMode::RosePine,
            ThemeMode::RosePine => ThemeMode::Cyberpunk,
            ThemeMode::Cyberpunk => ThemeMode::SunsetZen,
            ThemeMode::SunsetZen => ThemeMode::CatppuccinMocha,
        }
    }

    pub fn palette(&self) -> ThemePalette {
        match self {
            ThemeMode::CatppuccinMocha => ThemePalette {
                bg: hex_to_color(0x1e1e2e),
                surface: hex_to_color(0x181825),
                surface_bright: hex_to_color(0x313244),
                border: hex_to_color(0x45475a),
                text: hex_to_color(0xcdd6f4),
                subtext: hex_to_color(0xa6adc8),
                muted: hex_to_color(0x6c7086),
                accent: hex_to_color(0xcba6f7),     // Mauve
                primary: hex_to_color(0x89b4fa),    // Blue
                success: hex_to_color(0xa6e3a1),    // Green
                warning: hex_to_color(0xf9e2af),    // Yellow
                danger: hex_to_color(0xf38ba8),     // Red
                peach: hex_to_color(0xfab387),
            },
            ThemeMode::TokyoNight => ThemePalette {
                bg: hex_to_color(0x1a1b26),
                surface: hex_to_color(0x16161e),
                surface_bright: hex_to_color(0x24283b),
                border: hex_to_color(0x414868),
                text: hex_to_color(0xc0caf5),
                subtext: hex_to_color(0x9aa5ce),
                muted: hex_to_color(0x565f89),
                accent: hex_to_color(0xbb9af7),     // Purple
                primary: hex_to_color(0x7aa2f7),    // Blue
                success: hex_to_color(0x9ece6a),    // Green
                warning: hex_to_color(0xe0af68),    // Orange/Yellow
                danger: hex_to_color(0xf7768e),     // Red
                peach: hex_to_color(0xff9e64),
            },
            ThemeMode::NordSerenity => ThemePalette {
                bg: hex_to_color(0x2e3440),
                surface: hex_to_color(0x242933),
                surface_bright: hex_to_color(0x3b4252),
                border: hex_to_color(0x4c566a),
                text: hex_to_color(0xeceff4),
                subtext: hex_to_color(0xd8dee9),
                muted: hex_to_color(0x616e88),
                accent: hex_to_color(0x88c0d0),     // Frost cyan
                primary: hex_to_color(0x81a1c1),    // Frost blue
                success: hex_to_color(0xa3be8c),    // Sage green
                warning: hex_to_color(0xebcb8b),    // Yellow
                danger: hex_to_color(0xbf616a),     // Aurora red
                peach: hex_to_color(0xd08770),
            },
            ThemeMode::RosePine => ThemePalette {
                bg: hex_to_color(0x191724),
                surface: hex_to_color(0x12101a),
                surface_bright: hex_to_color(0x26233a),
                border: hex_to_color(0x403d52),
                text: hex_to_color(0xe0def4),
                subtext: hex_to_color(0x908caa),
                muted: hex_to_color(0x6e6a86),
                accent: hex_to_color(0xebbcba),     // Rose
                primary: hex_to_color(0xc4a7e7),    // Iris
                success: hex_to_color(0x9ccfd8),    // Foam
                warning: hex_to_color(0xf6c177),    // Gold
                danger: hex_to_color(0xeb6f92),     // Love red
                peach: hex_to_color(0xea9a97),
            },
            ThemeMode::Cyberpunk => ThemePalette {
                bg: hex_to_color(0x0b0e14),
                surface: hex_to_color(0x05070a),
                surface_bright: hex_to_color(0x151b23),
                border: hex_to_color(0x00ff9f),
                text: hex_to_color(0xf0f6fc),
                subtext: hex_to_color(0x8b949e),
                muted: hex_to_color(0x484f58),
                accent: hex_to_color(0x00ff9f),     // Neon Matrix Green
                primary: hex_to_color(0x00e5ff),    // Laser Cyan
                success: hex_to_color(0x39ff14),    // Neon Lime
                warning: hex_to_color(0xffe600),    // Electric Yellow
                danger: hex_to_color(0xff0055),     // Hot Pink/Red
                peach: hex_to_color(0xff8400),
            },
            ThemeMode::SunsetZen => ThemePalette {
                bg: hex_to_color(0x1a1625),
                surface: hex_to_color(0x130f1c),
                surface_bright: hex_to_color(0x2f2840),
                border: hex_to_color(0x493d62),
                text: hex_to_color(0xf5e6c8),
                subtext: hex_to_color(0xd5c4a1),
                muted: hex_to_color(0x7c6f64),
                accent: hex_to_color(0xff7597),     // Soft Sunset Coral
                primary: hex_to_color(0xffa07a),    // Peach
                success: hex_to_color(0xa8d8b9),    // Mint
                warning: hex_to_color(0xf6c177),    // Sunset Gold
                danger: hex_to_color(0xe06c75),     // Rose Red
                peach: hex_to_color(0xf39c12),
            },
        }
    }
}

pub struct ThemePalette {
    pub bg: Color,
    pub surface: Color,
    pub surface_bright: Color,
    pub border: Color,
    pub text: Color,
    pub subtext: Color,
    pub muted: Color,
    pub accent: Color,
    pub primary: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub peach: Color,
}

fn hex_to_color(hex: u32) -> Color {
    let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
    let b = (hex & 0xFF) as f32 / 255.0;
    Color::new(r, g, b, 1.0)
}
