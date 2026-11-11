//! Checkbox from md.comp.checkbox tokens: 18 pt container, 2 pt corner, 2 pt
//! unselected outline, 40 pt state layer, primary selected container.

use rune::{Color, El, el};

use crate::Theme;
use crate::components::{icons, state_layer};

/// Builder for a Material 3 checkbox. The caller owns the checked state and
/// attaches `.on_press(...)` to the returned `El`.
#[derive(Clone, Debug)]
pub struct Checkbox {
    checked: bool,
    indeterminate: bool,
    error: bool,
    label: String,
    key: Option<&'static str>,
    disabled: bool,
}

impl Checkbox {
    pub fn new(checked: bool, label: impl Into<String>) -> Self {
        Self {
            checked,
            indeterminate: false,
            error: false,
            label: label.into(),
            key: None,
            disabled: false,
        }
    }

    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Error styling from the checkbox error token group.
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
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
        let selected = self.checked || self.indeterminate;

        let (fill, border, icon_color) = if self.disabled {
            if selected {
                (Some(c.on_surface.alpha(0.38)), None, c.surface)
            } else {
                (None, Some(c.on_surface.alpha(0.38)), Color::TRANSPARENT)
            }
        } else if selected {
            let container = if self.error { c.error } else { c.primary };
            let ink = if self.error { c.on_error } else { c.on_primary };
            (Some(container), None, ink)
        } else if self.error {
            (None, Some(c.error), Color::TRANSPARENT)
        } else {
            (None, Some(c.on_surface_variant), Color::TRANSPARENT)
        };

        let mut control = el()
            .role("checkbox")
            .label(self.label.clone())
            .focusable(!self.disabled)
            .disabled(self.disabled)
            .size(18.0, 18.0)
            .radius(2.0)
            .center();

        if let Some(key) = self.key {
            control = control.key(key);
        }
        if self.indeterminate {
            control = control.checked_mixed();
        } else {
            control = control.checked(self.checked);
        }

        if let Some(fill) = fill {
            control = control.fill(fill);
        }
        if let Some(width) = border {
            control = control.border(2.0, width);
        }

        let (hover_color, pressed_color) = if self.disabled {
            (Color::TRANSPARENT, Color::TRANSPARENT)
        } else if self.error {
            (c.error, c.error)
        } else if selected {
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

        if selected {
            let mark = if self.indeterminate {
                &icons::REMOVE
            } else {
                &icons::CHECK
            };
            control = control.child(
                el().key("mark")
                    .a11y_hidden()
                    .center()
                    .child(rune::icon(mark, 18.0).color(icon_color)),
            );
        }

        control
    }
}
