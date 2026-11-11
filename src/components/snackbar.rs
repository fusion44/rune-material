//! Snackbars from md.comp.snackbar tokens. The action is a separate builder so
//! its press handler belongs to the app.

use rune::{El, row, text};

use crate::Theme;
use crate::components::{elevation, state_layer};
use crate::ink::{self, Ink};

/// Builder for the snackbar's text action. Attach `.on_press(..)` to the
/// returned element before passing it to [`Snackbar::action`].
#[derive(Clone, Debug)]
pub struct SnackbarAction {
    label: String,
    key: Option<&'static str>,
    ink: Option<Ink>,
}

impl SnackbarAction {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            key: None,
            ink: None,
        }
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    /// Material ink ripple on pointer press (see `rune_material::ink`). Needs
    /// `.key(..)`; occupies the element's `on_drag` slot for origin capture.
    pub fn ink(mut self, ink: &Ink) -> Self {
        self.ink = Some(ink.clone());
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let c = &theme.color;
        let mut action = row()
            .h(40.0)
            .px(12.0)
            .radius(theme.shape.corner_full)
            .center_y()
            .focusable(true)
            .hover(state_layer(c.inverse_primary, theme.state.hover))
            .press(state_layer(c.inverse_primary, theme.state.pressed))
            .focus_ring(3.0, 2.0, c.inverse_primary);

        action = ink::capture(&self.ink, self.key, false, action);
        // First child paints under the label.
        if let Some(wash) = ink::wash(
            &self.ink,
            self.key,
            state_layer(c.inverse_primary, theme.state.pressed),
            theme.shape.corner_full,
            Some([theme.shape.corner_full; 4]),
        ) {
            action = action.child(wash);
        }
        let mut label_el = text(self.label)
            .font(theme.type_scale.label_large)
            .color(c.inverse_primary);
        if let Some(key) = self.key {
            label_el = label_el.key(format!("{key}:label"));
        }
        action = action.child(label_el);
        if let Some(key) = self.key {
            action = action.key(key);
        }
        action
    }
}

/// Builder for a snackbar surface. Single-line is 48 pt high, two-line 68 pt;
/// both use inverse-surface and level 3.
#[derive(Clone)]
pub struct Snackbar {
    text: String,
    action: Option<El>,
    two_line: bool,
    width: Option<f32>,
    key: Option<&'static str>,
}

impl Snackbar {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            action: None,
            two_line: false,
            width: None,
            key: None,
        }
    }

    pub fn action(mut self, action: El) -> Self {
        self.action = Some(action);
        self
    }

    pub fn two_line(mut self, two_line: bool) -> Self {
        self.two_line = two_line;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let c = &theme.color;
        let has_action = self.action.is_some();

        let body = text(self.text)
            .font(theme.type_scale.body_medium)
            .color(c.inverse_on_surface)
            .wrap()
            .grow(1.0);

        let mut surface = row()
            .min_h(if self.two_line { 68.0 } else { 48.0 })
            .pl(16.0)
            .pr(if has_action { 8.0 } else { 16.0 })
            .py(if self.two_line { 14.0 } else { 0.0 })
            .gap(4.0)
            .center_y()
            .radius(theme.shape.corner_extra_small)
            .fill(c.inverse_surface)
            .child(body);
        if let Some(action) = self.action {
            surface = surface.child(action);
        }

        let mut snackbar = match self.width {
            Some(width) => surface.w(width),
            None => surface.min_w(344.0).max_w(672.0),
        };
        if let Some(key) = self.key {
            snackbar = snackbar.key(key);
        }
        elevation(snackbar, theme, 3)
    }
}
