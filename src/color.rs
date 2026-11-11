//! Material 3 colour roles (md.sys.color), exact values from material-web v0.192.

use rune::Color;
use rune::color::hex;

/// All M3 semantic colour roles for one appearance.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorScheme {
    pub background: Color,
    pub error: Color,
    pub error_container: Color,
    pub inverse_on_surface: Color,
    pub inverse_primary: Color,
    pub inverse_surface: Color,
    pub on_background: Color,
    pub on_error: Color,
    pub on_error_container: Color,
    pub on_primary: Color,
    pub on_primary_container: Color,
    pub on_primary_fixed: Color,
    pub on_primary_fixed_variant: Color,
    pub on_secondary: Color,
    pub on_secondary_container: Color,
    pub on_secondary_fixed: Color,
    pub on_secondary_fixed_variant: Color,
    pub on_surface: Color,
    pub on_surface_variant: Color,
    pub on_tertiary: Color,
    pub on_tertiary_container: Color,
    pub on_tertiary_fixed: Color,
    pub on_tertiary_fixed_variant: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub primary: Color,
    pub primary_container: Color,
    pub primary_fixed: Color,
    pub primary_fixed_dim: Color,
    pub scrim: Color,
    pub secondary: Color,
    pub secondary_container: Color,
    pub secondary_fixed: Color,
    pub secondary_fixed_dim: Color,
    pub shadow: Color,
    pub surface: Color,
    pub surface_bright: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
    pub surface_container_low: Color,
    pub surface_container_lowest: Color,
    pub surface_dim: Color,
    pub surface_tint: Color,
    pub surface_variant: Color,
    pub tertiary: Color,
    pub tertiary_container: Color,
    pub tertiary_fixed: Color,
    pub tertiary_fixed_dim: Color,
}

fn h(s: &str) -> Color {
    hex(s).expect("valid Material token hex")
}

/// Baseline light scheme from material-web `values-light`.
pub fn light() -> ColorScheme {
    ColorScheme {
        background: h("#fef7ffff"),
        error: h("#b3261e"),
        error_container: h("#f9dedc"),
        inverse_on_surface: h("#f5eff7"),
        inverse_primary: h("#d0bcffff"),
        inverse_surface: h("#322f35"),
        on_background: h("#1d1b20"),
        on_error: h("#fff"),
        on_error_container: h("#410e0b"),
        on_primary: h("#fff"),
        on_primary_container: h("#21005d"),
        on_primary_fixed: h("#21005d"),
        on_primary_fixed_variant: h("#4f378bff"),
        on_secondary: h("#fff"),
        on_secondary_container: h("#1d192b"),
        on_secondary_fixed: h("#1d192b"),
        on_secondary_fixed_variant: h("#4a4458"),
        on_surface: h("#1d1b20"),
        on_surface_variant: h("#49454fff"),
        on_tertiary: h("#fff"),
        on_tertiary_container: h("#31111d"),
        on_tertiary_fixed: h("#31111d"),
        on_tertiary_fixed_variant: h("#633b48"),
        outline: h("#79747e"),
        outline_variant: h("#cac4d0ff"),
        primary: h("#6750a4"),
        primary_container: h("#eaddffff"),
        primary_fixed: h("#eaddffff"),
        primary_fixed_dim: h("#d0bcffff"),
        scrim: h("#000"),
        secondary: h("#625b71"),
        secondary_container: h("#e8def8"),
        secondary_fixed: h("#e8def8"),
        secondary_fixed_dim: h("#ccc2dc"),
        shadow: h("#000"),
        surface: h("#fef7ffff"),
        surface_bright: h("#fef7ffff"),
        surface_container: h("#f3edf7"),
        surface_container_high: h("#ece6f0"),
        surface_container_highest: h("#e6e0e9"),
        surface_container_low: h("#f7f2fa"),
        surface_container_lowest: h("#fff"),
        surface_dim: h("#ded8e1"),
        surface_tint: h("#6750a4"),
        surface_variant: h("#e7e0ec"),
        tertiary: h("#7d5260"),
        tertiary_container: h("#ffd8e4"),
        tertiary_fixed: h("#ffd8e4"),
        tertiary_fixed_dim: h("#efb8c8"),
    }
}

/// Baseline dark scheme from material-web `values-dark`.
pub fn dark() -> ColorScheme {
    ColorScheme {
        background: h("#141218ff"),
        error: h("#f2b8b5ff"),
        error_container: h("#8c1d18"),
        inverse_on_surface: h("#322f35"),
        inverse_primary: h("#6750a4"),
        inverse_surface: h("#e6e0e9"),
        on_background: h("#e6e0e9"),
        on_error: h("#601410"),
        on_error_container: h("#f9dedc"),
        on_primary: h("#381e72ff"),
        on_primary_container: h("#eaddffff"),
        on_primary_fixed: h("#21005d"),
        on_primary_fixed_variant: h("#4f378bff"),
        on_secondary: h("#332d41"),
        on_secondary_container: h("#e8def8"),
        on_secondary_fixed: h("#1d192b"),
        on_secondary_fixed_variant: h("#4a4458"),
        on_surface: h("#e6e0e9"),
        on_surface_variant: h("#cac4d0ff"),
        on_tertiary: h("#492532"),
        on_tertiary_container: h("#ffd8e4"),
        on_tertiary_fixed: h("#31111d"),
        on_tertiary_fixed_variant: h("#633b48"),
        outline: h("#938f99ff"),
        outline_variant: h("#49454fff"),
        primary: h("#d0bcffff"),
        primary_container: h("#4f378bff"),
        primary_fixed: h("#eaddffff"),
        primary_fixed_dim: h("#d0bcffff"),
        scrim: h("#000"),
        secondary: h("#ccc2dc"),
        secondary_container: h("#4a4458"),
        secondary_fixed: h("#e8def8"),
        secondary_fixed_dim: h("#ccc2dc"),
        shadow: h("#000"),
        surface: h("#141218ff"),
        surface_bright: h("#3b383e"),
        surface_container: h("#211f26"),
        surface_container_high: h("#2b2930"),
        surface_container_highest: h("#36343b"),
        surface_container_low: h("#1d1b20"),
        surface_container_lowest: h("#0f0d13ff"),
        surface_dim: h("#141218ff"),
        surface_tint: h("#d0bcffff"),
        surface_variant: h("#49454fff"),
        tertiary: h("#efb8c8"),
        tertiary_container: h("#633b48"),
        tertiary_fixed: h("#ffd8e4"),
        tertiary_fixed_dim: h("#efb8c8"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_light_roles() {
        let c = light();
        assert_eq!(c.primary.hex(), "#6750a4ff");
        assert_eq!(c.on_primary.hex(), "#ffffffff");
        assert_eq!(c.primary_container.hex(), "#eaddffff");
        assert_eq!(c.surface.hex(), "#fef7ffff");
        assert_eq!(c.surface_container_lowest.hex(), "#ffffffff");
        assert_eq!(c.on_surface_variant.hex(), "#49454fff");
        assert_eq!(c.outline_variant.hex(), "#cac4d0ff");
        assert_eq!(c.error.hex(), "#b3261eff");
    }

    #[test]
    fn exact_dark_roles() {
        let c = dark();
        assert_eq!(c.primary.hex(), "#d0bcffff");
        assert_eq!(c.on_primary.hex(), "#381e72ff");
        assert_eq!(c.primary_container.hex(), "#4f378bff");
        assert_eq!(c.surface.hex(), "#141218ff");
        assert_eq!(c.surface_container_lowest.hex(), "#0f0d13ff");
        assert_eq!(c.on_surface_variant.hex(), "#cac4d0ff");
        assert_eq!(c.outline.hex(), "#938f99ff");
        assert_eq!(c.error.hex(), "#f2b8b5ff");
    }
}
