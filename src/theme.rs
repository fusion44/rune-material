//! A Material 3 theme: semantic colour roles plus type, shape, spacing, elevation,
//! state and motion tokens. Components read only these tokens.

use crate::color::{self, ColorScheme};
use crate::elevation::ElevationTokens;
use crate::motion::MotionTokens;
use crate::shape::{ShapeTokens, Spacing};
use crate::state::StateTokens;
use crate::typography::TypeScale;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub dark: bool,
    pub color: ColorScheme,
    pub type_scale: TypeScale,
    pub shape: ShapeTokens,
    pub spacing: Spacing,
    pub elevation: ElevationTokens,
    pub state: StateTokens,
    pub motion: MotionTokens,
}

impl Theme {
    pub fn light() -> Theme {
        Theme {
            dark: false,
            color: color::light(),
            type_scale: TypeScale::material(),
            shape: ShapeTokens::material(),
            spacing: Spacing::material(),
            elevation: ElevationTokens::material(),
            state: StateTokens::material(),
            motion: MotionTokens::material(),
        }
    }

    pub fn dark() -> Theme {
        Theme {
            dark: true,
            color: color::dark(),
            type_scale: TypeScale::material(),
            shape: ShapeTokens::material(),
            spacing: Spacing::material(),
            elevation: ElevationTokens::material(),
            state: StateTokens::material(),
            motion: MotionTokens::material(),
        }
    }

    pub fn with_scheme(scheme: ColorScheme, dark: bool) -> Theme {
        let mut theme = if dark { Theme::dark() } else { Theme::light() };
        theme.dark = dark;
        theme.color = scheme;
        theme
    }

    /// Baseline colour scheme from M3 tokens, light or dark.
    pub fn from_dark(dark: bool) -> Theme {
        if dark { Theme::dark() } else { Theme::light() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_themes_match_material_tokens() {
        let l = Theme::light();
        assert_eq!(l.color.primary.hex(), "#6750a4ff");
        assert_eq!(l.type_scale.label_large.size, 14.0);
        assert_eq!(l.shape.corner_full, 9999.0);
        assert!(!l.dark);

        let d = Theme::dark();
        assert_eq!(d.color.primary.hex(), "#d0bcffff");
        assert!(d.dark);
    }

    #[test]
    fn custom_scheme_keeps_other_tokens() {
        let mut scheme = color::light();
        scheme.primary = rune::Color::BLACK;
        let t = Theme::with_scheme(scheme, false);
        assert_eq!(t.color.primary.hex(), "#000000ff");
        assert_eq!(t.shape.corner_large, 16.0);
    }
}
