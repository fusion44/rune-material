//! Material 3 shape and spacing tokens.

/// md.sys.shape corners in points, plus per-corner variants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShapeTokens {
    pub corner_none: f32,
    pub corner_extra_small: f32,
    pub corner_small: f32,
    pub corner_medium: f32,
    pub corner_large: f32,
    pub corner_extra_large: f32,
    pub corner_full: f32,
}

impl ShapeTokens {
    pub const fn material() -> ShapeTokens {
        ShapeTokens {
            corner_none: 0.0,
            corner_extra_small: 4.0,
            corner_small: 8.0,
            corner_medium: 12.0,
            corner_large: 16.0,
            corner_extra_large: 28.0,
            corner_full: 9999.0,
        }
    }

    /// Top corners only: extra-small top, large top, extra-large top.
    pub fn corner_extra_small_top(self) -> [f32; 4] {
        [self.corner_extra_small, self.corner_extra_small, 0.0, 0.0]
    }
    pub fn corner_large_top(self) -> [f32; 4] {
        [self.corner_large, self.corner_large, 0.0, 0.0]
    }
    pub fn corner_extra_large_top(self) -> [f32; 4] {
        [self.corner_extra_large, self.corner_extra_large, 0.0, 0.0]
    }
    pub fn corner_large_start(self) -> [f32; 4] {
        [self.corner_large, 0.0, 0.0, self.corner_large]
    }
    pub fn corner_large_end(self) -> [f32; 4] {
        [0.0, self.corner_large, self.corner_large, 0.0]
    }
}

/// The 4 dp spacing grid and the common increments used by Material.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spacing {
    pub grid: f32,
    pub x1: f32,
    pub x2: f32,
    pub x3: f32,
    pub x4: f32,
    pub x6: f32,
    pub x8: f32,
    pub x12: f32,
    /// Minimum interactive target.
    pub touch_target: f32,
}

impl Spacing {
    pub const fn material() -> Spacing {
        Spacing {
            grid: 4.0,
            x1: 4.0,
            x2: 8.0,
            x3: 12.0,
            x4: 16.0,
            x6: 24.0,
            x8: 32.0,
            x12: 48.0,
            touch_target: 48.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_shapes_and_spacing() {
        let s = ShapeTokens::material();
        assert_eq!(
            [
                s.corner_none,
                s.corner_extra_small,
                s.corner_small,
                s.corner_medium,
                s.corner_large,
                s.corner_extra_large,
                s.corner_full
            ],
            [0.0, 4.0, 8.0, 12.0, 16.0, 28.0, 9999.0]
        );
        assert_eq!(s.corner_large_end(), [0.0, 16.0, 16.0, 0.0]);
        let sp = Spacing::material();
        assert_eq!(sp.grid, 4.0);
        assert_eq!(sp.touch_target, 48.0);
    }
}
