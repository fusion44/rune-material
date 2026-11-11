//! Small and large badges from md.comp.badge tokens.

use crate::Theme;
use rune::{El, el, text};

/// Builder for a Material 3 badge: a 6 pt dot without text, or a 16 pt pill
/// with a label-small count.
#[derive(Clone, Debug, Default)]
pub struct Badge {
    content: Option<String>,
    key: Option<&'static str>,
}

impl Badge {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add text, switching to the 16 pt large badge.
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let c = &theme.color;
        let Some(content) = self.content else {
            let mut dot = el()
                .size(6.0, 6.0)
                .radius(theme.shape.corner_full)
                .fill(c.error)
                .role("status");
            if let Some(key) = self.key {
                dot = dot.key(key);
            }
            return dot;
        };

        let mut badge = el()
            .h(16.0)
            .min_w(16.0)
            .px(4.0)
            .radius(theme.shape.corner_full)
            .fill(c.error)
            .center()
            .role("status")
            .label(content.clone())
            .child(
                text(content)
                    .font(theme.type_scale.label_small)
                    .color(c.on_error),
            );
        if let Some(key) = self.key {
            badge = badge.key(key);
        }
        badge
    }
}
