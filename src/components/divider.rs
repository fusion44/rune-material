//! Horizontal and vertical dividers from md.comp.divider tokens.

use crate::Theme;
use rune::{El, el};

/// Builder for a 1 pt divider in `outline-variant`.
#[derive(Clone, Debug)]
pub struct Divider {
    vertical: bool,
    inset_start: f32,
    inset_end: f32,
    key: Option<&'static str>,
}

impl Default for Divider {
    fn default() -> Self {
        Self {
            vertical: false,
            inset_start: 0.0,
            inset_end: 0.0,
            key: None,
        }
    }
}

impl Divider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// Inset from the start and end edges, along the divider's length.
    pub fn inset(mut self, inset: f32) -> Self {
        self.inset_start = inset;
        self.inset_end = inset;
        self
    }

    pub fn inset_start(mut self, inset: f32) -> Self {
        self.inset_start = inset;
        self
    }

    pub fn inset_end(mut self, inset: f32) -> Self {
        self.inset_end = inset;
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let mut divider = el().fill(theme.color.outline_variant);
        if self.vertical {
            divider = divider
                .w(1.0)
                .h_full()
                .mt(self.inset_start)
                .mb(self.inset_end);
        } else {
            divider = divider
                .h(1.0)
                .w_full()
                .ml(self.inset_start)
                .mr(self.inset_end);
        }
        if let Some(key) = self.key {
            divider = divider.key(key);
        }
        divider
    }
}
