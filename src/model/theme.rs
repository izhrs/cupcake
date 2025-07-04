// Theme is part of the model/state so theme can be changed at runtime/realtime.

use ratatui::style::Color;

// TODO: make propeties more descriptive instead of using shadcn like names
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub background: Option<Color>,
    pub forground: Color,

    pub border: Color,
    pub border_active: Color,

    pub primary: Color,
    pub primary_forground: Color,

    pub muted: Color,
    pub muted_forground: Color,

    pub destructive: Color,
    pub destructive_forground: Color,

    pub success: Color,
    pub success_forground: Color,

    pub warning: Color,
    pub warning_forground: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::catppuccin_mocha().transparent()
    }
}

impl Theme {
    /// Sets the background to transparent
    pub fn transparent(mut self) -> Self {
        self.background = None;
        self
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            background: Some(Color::from_u32(0x1e1e2e)),
            forground: Color::from_u32(0xcdd6f4),

            border: Color::from_u32(0x6c7086),
            border_active: Color::from_u32(0xcba6f7),

            primary: Color::from_u32(0xcba6f7),
            primary_forground: Color::from_u32(0x1e1e2e),

            muted: Color::from_u32(0x6c7086),
            muted_forground: Color::from_u32(0xa6adc8),

            destructive: Color::from_u32(0xf38ba8),
            destructive_forground: Color::from_u32(0x1e1e2e),

            success: Color::from_u32(0xa6e3a1),
            success_forground: Color::from_u32(0x1e1e2e),

            warning: Color::from_u32(0xf9e2af),
            warning_forground: Color::from_u32(0x1e1e2e),
        }
    }

    pub fn catppuccin_latte() -> Self {
        Self {
            background: Some(Color::from_u32(0xeff1f5)),
            forground: Color::from_u32(0x4c4f69),

            border: Color::from_u32(0xccd0da),
            border_active: Color::from_u32(0x8839ef),

            primary: Color::from_u32(0x8839ef),
            primary_forground: Color::from_u32(0xeff1f5),

            muted: Color::from_u32(0x7c7f93),
            muted_forground: Color::from_u32(0x5c5f77),

            destructive: Color::from_u32(0xd20f39),
            destructive_forground: Color::from_u32(0xeff1f5),

            success: Color::from_u32(0x40a02b),
            success_forground: Color::from_u32(0xeff1f5),

            warning: Color::from_u32(0xdf8e1d),
            warning_forground: Color::from_u32(0xeff1f5),
        }
    }

    pub fn dracula() -> Self {
        Self {
            background: Some(Color::from_u32(0x282a36)),
            forground: Color::from_u32(0xf8f8f2),

            border: Color::from_u32(0x6272a4),
            border_active: Color::from_u32(0xbd93f9),

            primary: Color::from_u32(0xbd93f9),
            primary_forground: Color::from_u32(0x282a36),

            muted: Color::from_u32(0x6272a4),
            muted_forground: Color::from_u32(0xf8f8f2),

            destructive: Color::from_u32(0xff5555),
            destructive_forground: Color::from_u32(0x282a36),

            success: Color::from_u32(0x50fa7b),
            success_forground: Color::from_u32(0x282a36),

            warning: Color::from_u32(0xf1fa8c),
            warning_forground: Color::from_u32(0x282a36),
        }
    }

    pub fn rose_pine() -> Self {
        Self {
            background: Some(Color::from_u32(0x191724)),
            forground: Color::from_u32(0xe0def4),

            border: Color::from_u32(0x403d52),
            border_active: Color::from_u32(0xc4a7e7),

            primary: Color::from_u32(0xc4a7e7),
            primary_forground: Color::from_u32(0x191724),

            muted: Color::from_u32(0x6e6a86),
            muted_forground: Color::from_u32(0x908caa),

            destructive: Color::from_u32(0xeb6f92),
            destructive_forground: Color::from_u32(0x191724),

            success: Color::from_u32(0x9ccfd8),
            success_forground: Color::from_u32(0x191724),

            warning: Color::from_u32(0xf6c177),
            warning_forground: Color::from_u32(0x191724),
        }
    }

    pub fn nord() -> Self {
        Self {
            background: Some(Color::from_u32(0x2e3440)),
            forground: Color::from_u32(0xd8dee9),

            border: Color::from_u32(0x4c566a),
            border_active: Color::from_u32(0x88c0d0),

            primary: Color::from_u32(0x5e81ac),
            primary_forground: Color::from_u32(0x2e3440),

            muted: Color::from_u32(0x4c566a),
            muted_forground: Color::from_u32(0xd8dee9),

            destructive: Color::from_u32(0xbf616a),
            destructive_forground: Color::from_u32(0x2e3440),

            success: Color::from_u32(0xa3be8c),
            success_forground: Color::from_u32(0x2e3440),

            warning: Color::from_u32(0xebcb8b),
            warning_forground: Color::from_u32(0x2e3440),
        }
    }
}
