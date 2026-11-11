//! Material 3 typescale (md.sys.typescale), exact values from material-web v0.192.
//! Material Web states these in rem with a 16 px root; they are converted to points.

use rune::{Family, TextStyle};

mod font_scale;
pub use font_scale::FontScale;

const ROBOTO: Family = Family::Named("Roboto");

const fn style(size: f32, weight: f32, line_height: f32, tracking: f32) -> TextStyle {
    TextStyle::new(size, weight)
        .family(ROBOTO)
        .lh(line_height)
        .tracking(tracking)
}

/// The fifteen M3 typescale roles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeScale {
    pub body_large: TextStyle,
    pub body_medium: TextStyle,
    pub body_small: TextStyle,
    pub display_large: TextStyle,
    pub display_medium: TextStyle,
    pub display_small: TextStyle,
    pub headline_large: TextStyle,
    pub headline_medium: TextStyle,
    pub headline_small: TextStyle,
    pub label_large: TextStyle,
    pub label_medium: TextStyle,
    pub label_small: TextStyle,
    pub title_large: TextStyle,
    pub title_medium: TextStyle,
    pub title_small: TextStyle,
}

impl TypeScale {
    /// Scale text metrics together, preserving weights and font families.
    pub fn scaled(mut self, factor: f32) -> Self {
        if !factor.is_finite() || factor <= 0.0 {
            return self;
        }
        for style in [
            &mut self.body_large,
            &mut self.body_medium,
            &mut self.body_small,
            &mut self.display_large,
            &mut self.display_medium,
            &mut self.display_small,
            &mut self.headline_large,
            &mut self.headline_medium,
            &mut self.headline_small,
            &mut self.label_large,
            &mut self.label_medium,
            &mut self.label_small,
            &mut self.title_large,
            &mut self.title_medium,
            &mut self.title_small,
        ] {
            style.size *= factor;
            style.line_height *= factor;
            style.tracking *= factor;
        }
        self
    }

    pub const fn material() -> TypeScale {
        TypeScale {
            // 1rem/1.5rem, tracking .03125rem, regular
            body_large: style(16.0, 400.0, 24.0, 0.5),
            // .875rem/1.25rem, tracking .015625rem, regular
            body_medium: style(14.0, 400.0, 20.0, 0.25),
            // .75rem/1rem, tracking .025rem, regular
            body_small: style(12.0, 400.0, 16.0, 0.4),
            // 3.5625rem/4rem, tracking -.015625rem, regular
            display_large: style(57.0, 400.0, 64.0, -0.25),
            // 2.8125rem/3.25rem, tracking 0, regular
            display_medium: style(45.0, 400.0, 52.0, 0.0),
            // 2.25rem/2.75rem, tracking 0, regular
            display_small: style(36.0, 400.0, 44.0, 0.0),
            // 2rem/2.5rem, tracking 0, regular
            headline_large: style(32.0, 400.0, 40.0, 0.0),
            // 1.75rem/2.25rem, tracking 0, regular
            headline_medium: style(28.0, 400.0, 36.0, 0.0),
            // 1.5rem/2rem, tracking 0, regular
            headline_small: style(24.0, 400.0, 32.0, 0.0),
            // .875rem/1.25rem, tracking .00625rem, medium
            label_large: style(14.0, 500.0, 20.0, 0.1),
            // .75rem/1rem, tracking .03125rem, medium
            label_medium: style(12.0, 500.0, 16.0, 0.5),
            // .6875rem/1rem, tracking .03125rem, medium
            label_small: style(11.0, 500.0, 16.0, 0.5),
            // 1.375rem/1.75rem, tracking 0, regular
            title_large: style(22.0, 400.0, 28.0, 0.0),
            // 1rem/1.5rem, tracking .009375rem, medium
            title_medium: style(16.0, 500.0, 24.0, 0.15),
            // .875rem/1.25rem, tracking .00625rem, medium
            title_small: style(14.0, 500.0, 20.0, 0.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_typescale_values() {
        let t = TypeScale::material();
        let cases = [
            (t.body_large, 16.0, 400.0, 24.0, 0.5),
            (t.body_medium, 14.0, 400.0, 20.0, 0.25),
            (t.body_small, 12.0, 400.0, 16.0, 0.4),
            (t.display_large, 57.0, 400.0, 64.0, -0.25),
            (t.display_medium, 45.0, 400.0, 52.0, 0.0),
            (t.display_small, 36.0, 400.0, 44.0, 0.0),
            (t.headline_large, 32.0, 400.0, 40.0, 0.0),
            (t.headline_medium, 28.0, 400.0, 36.0, 0.0),
            (t.headline_small, 24.0, 400.0, 32.0, 0.0),
            (t.label_large, 14.0, 500.0, 20.0, 0.1),
            (t.label_medium, 12.0, 500.0, 16.0, 0.5),
            (t.label_small, 11.0, 500.0, 16.0, 0.5),
            (t.title_large, 22.0, 400.0, 28.0, 0.0),
            (t.title_medium, 16.0, 500.0, 24.0, 0.15),
            (t.title_small, 14.0, 500.0, 20.0, 0.1),
        ];
        for (s, size, weight, lh, tracking) in cases {
            assert_eq!(s.size, size);
            assert_eq!(s.weight, weight);
            assert_eq!(s.line_height, lh);
            assert_eq!(s.tracking, tracking);
            assert_eq!(s.family, ROBOTO);
        }
    }

    #[test]
    fn scaling_preserves_family_and_weight_and_scales_all_metrics() {
        let baseline = TypeScale::material();
        let scaled = baseline.scaled(1.5);
        assert_eq!(scaled.body_medium.size, 21.0);
        assert_eq!(scaled.body_medium.line_height, 30.0);
        assert_eq!(scaled.body_medium.tracking, 0.375);
        assert_eq!(scaled.body_medium.family, baseline.body_medium.family);
        assert_eq!(scaled.label_large.weight, baseline.label_large.weight);
        assert_eq!(scaled.display_large.size, 85.5);
        assert_eq!(scaled.title_small.line_height, 30.0);
        for invalid in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert_eq!(baseline.scaled(invalid), baseline);
        }
    }
}
