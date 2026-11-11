//! Material 3 elevation tokens (md.sys.elevation) and the two-layer shadow recipe from
//! material-web's `elevation/internal/_elevation.scss`: a key shadow at 0.30 and an
//! ambient shadow at 0.15, both clipped by the element's shape.

use rune::{Color, Shadow};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ElevationTokens {
    pub level0: u8,
    pub level1: u8,
    pub level2: u8,
    pub level3: u8,
    pub level4: u8,
    pub level5: u8,
}

impl ElevationTokens {
    pub const fn material() -> ElevationTokens {
        ElevationTokens {
            level0: 0,
            level1: 1,
            level2: 3,
            level3: 6,
            level4: 8,
            level5: 12,
        }
    }

    /// Two-layer M3 shadow for levels 1..5; `None` for level 0.
    ///
    /// Key shadow (Material Web `.shadow::before`):
    /// level1 `0 1 2 0`, level2 `0 1 2 0`, level3 `0 1 3 0`, level4 `0 2 3 0`,
    /// level5 `0 4 4 0`.
    /// Ambient shadow (`.shadow::after`):
    /// level1 `0 1 3 1`, level2 `0 2 6 2`, level3 `0 4 8 3`, level4 `0 6 10 4`,
    /// level5 `0 8 12 6`.
    pub fn shadow(self, level: u8, shadow_color: Color) -> Option<[Shadow; 2]> {
        let (key_y, key_blur, ambient_y, ambient_blur, ambient_spread) = match level {
            1 => (1.0, 2.0, 1.0, 3.0, 1.0),
            2 => (1.0, 2.0, 2.0, 6.0, 2.0),
            3 => (1.0, 3.0, 4.0, 8.0, 3.0),
            4 => (2.0, 3.0, 6.0, 10.0, 4.0),
            5 => (4.0, 4.0, 8.0, 12.0, 6.0),
            _ => return None,
        };

        let key = Shadow {
            x: 0.0,
            y: key_y,
            blur: key_blur,
            spread: 0.0,
            color: shadow_color.alpha(0.30),
            inset: false,
        };
        let ambient = Shadow {
            x: 0.0,
            y: ambient_y,
            blur: ambient_blur,
            spread: ambient_spread,
            color: shadow_color.alpha(0.15),
            inset: false,
        };
        Some([key, ambient])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_level_values() {
        let e = ElevationTokens::material();
        assert_eq!(
            [e.level0, e.level1, e.level2, e.level3, e.level4, e.level5],
            [0, 1, 3, 6, 8, 12]
        );
        assert!(e.shadow(0, Color::BLACK).is_none());
    }

    #[test]
    fn two_layer_recipes_match_material_web() {
        let e = ElevationTokens::material();
        let [key, ambient] = e.shadow(1, Color::BLACK).unwrap();
        assert_eq!((key.y, key.blur, key.spread), (1.0, 2.0, 0.0));
        assert_eq!(key.color.hex(), "#0000004d");
        assert_eq!((ambient.y, ambient.blur, ambient.spread), (1.0, 3.0, 1.0));
        assert_eq!(ambient.color.hex(), "#00000026");

        let [key3, amb3] = e.shadow(3, Color::BLACK).unwrap();
        assert_eq!((key3.y, key3.blur), (1.0, 3.0));
        assert_eq!((amb3.y, amb3.blur, amb3.spread), (4.0, 8.0, 3.0));
    }
}
