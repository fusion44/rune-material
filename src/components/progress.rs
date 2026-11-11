//! Linear progress indicators from md.comp.linear-progress-indicator tokens.

use crate::Theme;
use rune::{El, el};

/// Builder for a determinate 4 pt linear progress indicator. `value` is a
/// fraction from 0 to 1.
#[derive(Clone, Debug)]
pub struct Progress {
    value: f32,
    width: Option<f32>,
    label: Option<String>,
    key: Option<&'static str>,
}

impl Progress {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            width: None,
            label: None,
            key: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let c = &theme.color;
        let mut track = el()
            .h(4.0)
            .radius(theme.shape.corner_none)
            .fill(c.surface_container_highest)
            .role("progressbar")
            .range_value(self.value as f64, 0.0, 1.0);
        track = match self.width {
            Some(width) => track.w(width),
            None => track.w_full(),
        };
        if let Some(label) = self.label {
            track = track.label(label);
        }
        if let Some(key) = self.key {
            track = track.key(key);
        }

        let mut active = el()
            .key("active")
            .absolute(0.0, 0.0)
            .h(4.0)
            .radius(theme.shape.corner_none)
            .fill(c.primary)
            .a11y_hidden();
        active = match self.width {
            Some(width) => active.w(width * self.value),
            None => active.w_frac(self.value),
        };
        track.child(active)
    }
}
