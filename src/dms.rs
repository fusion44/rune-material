//! DMS palette integration: map `dms-colors.json` into [`ColorScheme`]/[`Theme`].
//!
//! DMS does not broadcast palette changes over IPC. The shell watches the
//! generated palette file (`Theme.qml` uses `FileView.watchChanges` on
//! `$XDG_CACHE_HOME/dms/dms-colors.json`, written atomically via `.tmp` +
//! rename). This module does the mapping side; file watching lives in the
//! example apps so the reusable crate stays host-agnostic and dependency-light
//! apart from `serde_json`.

use std::path::PathBuf;

use crate::Theme;
use crate::color::{self, ColorScheme};

/// Dux still publishes typography under the legacy `dms` cache directory.
/// `$DMS_TYPOGRAPHY_FILE` overrides the XDG cache path.
pub fn typography_path() -> PathBuf {
    if let Some(path) = std::env::var_os("DMS_TYPOGRAPHY_FILE") {
        return PathBuf::from(path);
    }
    let cache = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".cache")
        });
    cache.join("dms/dms-typography.json")
}

/// Parse Dux's global scale (not the independent per-bar fontScale).
/// Accept the shell's 75–150% range; malformed settings keep the last scale.
pub fn font_scale_from_json(json: &[u8]) -> Option<f32> {
    let root: serde_json::Value = serde_json::from_slice(json).ok()?;
    let scale = root.get("fontScale")?.as_f64()?;
    (scale.is_finite() && (0.75..=1.5).contains(&scale)).then_some(scale as f32)
}

/// Resolve the palette file: `$DMS_COLORS_FILE`, else
/// `$XDG_CACHE_HOME/dms/dms-colors.json`, else `$HOME/.cache/dms/dms-colors.json`.
pub fn palette_path() -> PathBuf {
    if let Some(path) = std::env::var_os("DMS_COLORS_FILE") {
        return PathBuf::from(path);
    }
    if let Some(cache) = std::env::var_os("XDG_CACHE_HOME") {
        let mut path = PathBuf::from(cache);
        path.push("dms/dms-colors.json");
        return path;
    }
    let mut path = PathBuf::from(std::env::var_os("HOME").unwrap_or_default());
    path.push(".cache/dms/dms-colors.json");
    path
}

/// Parse one appearance (`colors.dark` / `colors.light`) into a scheme.
/// Missing roles fall back to the M3 baseline scheme for that appearance.
pub fn scheme_from_json(json: &[u8], dark: bool) -> Option<ColorScheme> {
    let root: serde_json::Value = serde_json::from_slice(json).ok()?;
    let map = root
        .get("colors")?
        .get(if dark { "dark" } else { "light" })?
        .as_object()?;
    let base = if dark { color::dark() } else { color::light() };
    let role = |key: &str, fallback: rune::Color| -> rune::Color {
        map.get(key)
            .and_then(|v| v.as_str())
            .and_then(rune::color::hex)
            .unwrap_or(fallback)
    };
    Some(ColorScheme {
        background: role("background", base.background),
        error: role("error", base.error),
        error_container: role("error_container", base.error_container),
        inverse_on_surface: role("inverse_on_surface", base.inverse_on_surface),
        inverse_primary: role("inverse_primary", base.inverse_primary),
        inverse_surface: role("inverse_surface", base.inverse_surface),
        on_background: role("on_background", base.on_background),
        on_error: role("on_error", base.on_error),
        on_error_container: role("on_error_container", base.on_error_container),
        on_primary: role("on_primary", base.on_primary),
        on_primary_container: role("on_primary_container", base.on_primary_container),
        on_primary_fixed: role("on_primary_fixed", base.on_primary_fixed),
        on_primary_fixed_variant: role("on_primary_fixed_variant", base.on_primary_fixed_variant),
        on_secondary: role("on_secondary", base.on_secondary),
        on_secondary_container: role("on_secondary_container", base.on_secondary_container),
        on_secondary_fixed: role("on_secondary_fixed", base.on_secondary_fixed),
        on_secondary_fixed_variant: role(
            "on_secondary_fixed_variant",
            base.on_secondary_fixed_variant,
        ),
        on_surface: role("on_surface", base.on_surface),
        on_surface_variant: role("on_surface_variant", base.on_surface_variant),
        on_tertiary: role("on_tertiary", base.on_tertiary),
        on_tertiary_container: role("on_tertiary_container", base.on_tertiary_container),
        on_tertiary_fixed: role("on_tertiary_fixed", base.on_tertiary_fixed),
        on_tertiary_fixed_variant: role(
            "on_tertiary_fixed_variant",
            base.on_tertiary_fixed_variant,
        ),
        outline: role("outline", base.outline),
        outline_variant: role("outline_variant", base.outline_variant),
        primary: role("primary", base.primary),
        primary_container: role("primary_container", base.primary_container),
        primary_fixed: role("primary_fixed", base.primary_fixed),
        primary_fixed_dim: role("primary_fixed_dim", base.primary_fixed_dim),
        scrim: role("scrim", base.scrim),
        secondary: role("secondary", base.secondary),
        secondary_container: role("secondary_container", base.secondary_container),
        secondary_fixed: role("secondary_fixed", base.secondary_fixed),
        secondary_fixed_dim: role("secondary_fixed_dim", base.secondary_fixed_dim),
        shadow: role("shadow", base.shadow),
        surface: role("surface", base.surface),
        surface_bright: role("surface_bright", base.surface_bright),
        surface_container: role("surface_container", base.surface_container),
        surface_container_high: role("surface_container_high", base.surface_container_high),
        surface_container_highest: role(
            "surface_container_highest",
            base.surface_container_highest,
        ),
        surface_container_low: role("surface_container_low", base.surface_container_low),
        surface_container_lowest: role("surface_container_lowest", base.surface_container_lowest),
        surface_dim: role("surface_dim", base.surface_dim),
        surface_tint: role("surface_tint", base.surface_tint),
        surface_variant: role("surface_variant", base.surface_variant),
        tertiary: role("tertiary", base.tertiary),
        tertiary_container: role("tertiary_container", base.tertiary_container),
        tertiary_fixed: role("tertiary_fixed", base.tertiary_fixed),
        tertiary_fixed_dim: role("tertiary_fixed_dim", base.tertiary_fixed_dim),
    })
}

/// Whether the top-level `mode` selects the dark appearance. Anything other
/// than `"light"` (including `"dark"`, `"smart"` and missing) is dark, matching
/// the shell's default.
pub fn mode_is_dark(json: &[u8]) -> bool {
    let Ok(root) = serde_json::from_slice::<serde_json::Value>(json) else {
        return false;
    };
    root.get("mode").and_then(|v| v.as_str()) != Some("light")
}

/// Parse a full [`Theme`], honouring top-level `mode` unless `dark` overrides it.
/// Returns the theme and the resolved appearance.
pub fn theme_from_json(json: &[u8], dark: Option<bool>) -> Option<(Theme, bool)> {
    let resolved = dark.unwrap_or_else(|| mode_is_dark(json));
    let scheme = scheme_from_json(json, resolved)?;
    Some((Theme::with_scheme(scheme, resolved), resolved))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r##"{
        "mode": "dark",
        "colors": {
            "dark": {
                "primary": "#d0bcff",
                "on_primary": "#381e72",
                "surface": "#141218",
                "on_surface": "#e6e0e9",
                "surface_container_high": "#2b2930"
            },
            "light": {
                "primary": "#6750a4",
                "on_primary": "#ffffff",
                "surface": "#fef7ff",
                "on_surface": "#1d1b20",
                "surface_container_high": "#ece6f0"
            }
        }
    }"##;

    #[test]
    fn honours_mode_and_maps_roles() {
        let (theme, dark) = theme_from_json(FIXTURE.as_bytes(), None).expect("parse");
        assert!(dark);
        assert!(theme.dark);
        assert_eq!(theme.color.primary.hex(), "#d0bcffff");
        assert_eq!(theme.color.surface_container_high.hex(), "#2b2930ff");
        // Missing roles fall back to baseline dark.
        assert_eq!(theme.color.error.hex(), color::dark().error.hex());
    }

    #[test]
    fn explicit_appearance_overrides_mode() {
        let (theme, dark) = theme_from_json(FIXTURE.as_bytes(), Some(false)).expect("parse");
        assert!(!dark);
        assert_eq!(theme.color.primary.hex(), "#6750a4ff");
    }

    #[test]
    fn rejects_invalid_json() {
        assert!(theme_from_json(b"not json", None).is_none());
        assert!(theme_from_json(b"{}", None).is_none());
    }

    #[test]
    fn typography_reads_only_the_global_scale() {
        assert_eq!(font_scale_from_json(br#"{"fontScale":1.2}"#), Some(1.2));
        assert_eq!(font_scale_from_json(br#"{"fontScale":0.75}"#), Some(0.75));
        assert_eq!(font_scale_from_json(br#"{"fontScale":1.5}"#), Some(1.5));
        for invalid in [
            br#"{}"#.as_slice(),
            br#"{"fontScale":"1.2"}"#,
            br#"{"fontScale":0}"#,
            br#"{"fontScale":-1}"#,
            br#"{"fontScale":100}"#,
            br#"{"fontScale":null}"#,
            br#"{"barConfigs":[{"fontScale":1.2}]}"#,
            b"broken json",
        ] {
            assert_eq!(font_scale_from_json(invalid), None);
        }
    }
}
