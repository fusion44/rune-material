//! Radio button from md.comp.radio-button tokens: 20 pt ring, 10 pt inner dot,
//! 40 pt state layer, primary selected ring and on-surface unselected ring.

use rune::{Color, El, el};

use crate::Theme;
use crate::components::state_layer;

/// Builder for a Material 3 radio button. The caller owns the checked state and
/// attaches `.on_press(...)` to the returned `El`.
#[derive(Clone, Debug)]
pub struct Radio {
    checked: bool,
    label: String,
    key: Option<&'static str>,
    disabled: bool,
}

impl Radio {
    pub fn new(checked: bool, label: impl Into<String>) -> Self {
        Self {
            checked,
            label: label.into(),
            key: None,
            disabled: false,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let c = &theme.color;

        let ring = if self.disabled {
            c.on_surface.alpha(0.38)
        } else if self.checked {
            c.primary
        } else {
            c.on_surface_variant
        };

        let mut control = el()
            .role("radio")
            .label(self.label.clone())
            .checked(self.checked)
            .focusable(!self.disabled)
            .disabled(self.disabled)
            .size(20.0, 20.0)
            .radius(theme.shape.corner_full)
            .border(2.0, ring);

        if let Some(key) = self.key {
            control = control.key(key);
        }

        if self.checked {
            control = control.child(
                el().key("dot")
                    .a11y_hidden()
                    .size(10.0, 10.0)
                    .radius(theme.shape.corner_full)
                    .absolute(5.0, 5.0)
                    .fill(ring),
            );
        }

        let (hover_color, pressed_color) = if self.disabled {
            (Color::TRANSPARENT, Color::TRANSPARENT)
        } else if self.checked {
            (c.primary, c.on_surface)
        } else {
            (c.on_surface, c.primary)
        };

        if self.disabled {
            control = control
                .hover(Color::TRANSPARENT)
                .press(Color::TRANSPARENT)
                .no_focus_ring();
        } else {
            control = control
                .hover(state_layer(hover_color, theme.state.hover))
                .press(state_layer(pressed_color, theme.state.pressed))
                .focus_ring(2.0, 1.0, c.secondary);
        }

        control
    }
}
