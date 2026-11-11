//! Dialogs and their scrim from md.comp.dialog and md.comp.scrim tokens. The
//! builder returns the panel only: the app mounts it in an overlay layer and
//! attaches `.modal(..)` for focus trapping and dismissal.

use rune::{El, Icon, col, row, text};

use crate::Theme;
use crate::components::elevation;

/// Builder for a Material 3 dialog panel (280-560 pt wide, 28 pt corners,
/// surface-container-high, level 3).
#[derive(Clone)]
pub struct Dialog {
    headline: Option<String>,
    supporting_text: Option<String>,
    icon: Option<&'static Icon>,
    children: Vec<El>,
    actions: Vec<El>,
    min_width: f32,
    max_width: f32,
    min_height: f32,
    key: Option<&'static str>,
}

impl Default for Dialog {
    fn default() -> Self {
        Self {
            headline: None,
            supporting_text: None,
            icon: None,
            children: Vec::new(),
            actions: Vec::new(),
            min_width: 280.0,
            max_width: 560.0,
            min_height: 140.0,
            key: None,
        }
    }
}

impl Dialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn headline(mut self, headline: impl Into<String>) -> Self {
        self.headline = Some(headline.into());
        self
    }

    pub fn supporting_text(mut self, supporting_text: impl Into<String>) -> Self {
        self.supporting_text = Some(supporting_text.into());
        self
    }

    /// A 24 pt secondary icon above the headline.
    pub fn icon(mut self, icon: &'static Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn child(mut self, child: El) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = El>) -> Self {
        self.children.extend(children);
        self
    }

    /// A trailing action, usually a text button. Attach its press handler before
    /// adding it.
    pub fn action(mut self, action: El) -> Self {
        self.actions.push(action);
        self
    }

    pub fn min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.min_width = width;
        self.max_width = width;
        self
    }

    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height;
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let c = &theme.color;

        let mut dialog = col()
            .min_w(self.min_width)
            .max_w(self.max_width)
            .min_h(self.min_height)
            .radius(theme.shape.corner_extra_large)
            .fill(c.surface_container_high)
            .role("dialog");
        if let Some(key) = self.key {
            dialog = dialog.key(key);
        }
        if let Some(headline) = &self.headline {
            dialog = dialog.label(headline.clone());
        }
        dialog = elevation(dialog, theme, 3);

        if let Some(icon) = self.icon {
            dialog = dialog.child(
                row()
                    .w_full()
                    .justify_center()
                    .pt(24.0)
                    .child(rune::icon(icon, 24.0).color(c.secondary))
                    .a11y_hidden(),
            );
        }

        if let Some(headline) = self.headline {
            dialog = dialog.child(
                col()
                    .w_full()
                    .px(24.0)
                    .pt(if self.icon.is_some() { 16.0 } else { 24.0 })
                    .child(
                        text(headline)
                            .font(theme.type_scale.headline_small)
                            .color(c.on_surface)
                            .wrap(),
                    ),
            );
        }

        if let Some(supporting) = self.supporting_text {
            dialog = dialog.child(
                col().w_full().px(24.0).pt(16.0).child(
                    text(supporting)
                        .font(theme.type_scale.body_medium)
                        .color(c.on_surface_variant)
                        .wrap(),
                ),
            );
        }

        if !self.children.is_empty() {
            dialog = dialog.child(col().w_full().px(24.0).pt(16.0).children(self.children));
        }

        if !self.actions.is_empty() {
            dialog = dialog.child(
                row()
                    .w_full()
                    .justify_end()
                    .gap(8.0)
                    .pt(16.0)
                    .px(24.0)
                    .pb(24.0)
                    // Arrow keys rove across the actions; Tab visits them.
                    .focus_group()
                    .children(self.actions),
            );
        }

        dialog
    }
}

/// Builder for the dialog scrim: `scrim` at 32% opacity, optionally frosted.
#[derive(Clone, Debug, Default)]
pub struct Scrim {
    key: Option<&'static str>,
    backdrop_blur: f32,
}

impl Scrim {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    /// Blur the content behind the scrim, not the dialog on top. Defaults to
    /// zero to preserve the standard Material scrim; sigma is in points.
    pub fn backdrop_blur(mut self, sigma: f32) -> Self {
        self.backdrop_blur = sigma;
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let mut scrim = rune::el()
            .w_full()
            .h_full()
            .fill(theme.color.scrim.alpha(0.32))
            .backdrop_blur(self.backdrop_blur)
            .a11y_hidden();
        if let Some(key) = self.key {
            scrim = scrim.key(key);
        }
        scrim
    }
}
