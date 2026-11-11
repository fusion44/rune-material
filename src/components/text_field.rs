//! Filled and outlined text fields from md.comp.*-text-field tokens.
//!
//! The builder returns the input element itself, so an app owns the edit and
//! attaches `.on_edit(..)` to the result. The label, active indicator, icons and
//! supporting text are overlays on that input root.

use rune::{El, Icon, TextEdit, el, text, text_input};

use crate::Theme;
use crate::components::state_layer;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TextFieldVariant {
    #[default]
    Filled,
    Outlined,
}

/// Builder for a Material 3 single-line text field.
///
/// Each live input needs its own `TextEdit` in app state: the framework
/// recognises a field across rebuilds by its model, so two fields sharing
/// one model flap focus between them after every edit. Display-only copies
/// get their own model too.
#[derive(Clone, Debug)]
pub struct TextField {
    edit: TextEdit,
    label: String,
    placeholder: String,
    variant: TextFieldVariant,
    supporting_text: Option<String>,
    error: bool,
    disabled: bool,
    leading_icon: Option<&'static Icon>,
    trailing_icon: Option<&'static Icon>,
    width: Option<f32>,
    key: Option<&'static str>,
}

impl TextField {
    pub fn new(edit: &TextEdit, label: impl Into<String>) -> Self {
        Self {
            edit: edit.clone(),
            label: label.into(),
            placeholder: String::new(),
            variant: TextFieldVariant::Filled,
            supporting_text: None,
            error: false,
            disabled: false,
            leading_icon: None,
            trailing_icon: None,
            width: None,
            key: None,
        }
    }

    pub fn variant(mut self, variant: TextFieldVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn supporting_text(mut self, text: impl Into<String>) -> Self {
        self.supporting_text = Some(text.into());
        self
    }

    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// A 24 pt leading icon, with 12 pt leading space and 16 pt content space.
    pub fn leading_icon(mut self, icon: &'static Icon) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    /// A 24 pt trailing icon, with 12 pt trailing space and 16 pt content space.
    pub fn trailing_icon(mut self, icon: &'static Icon) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    /// Field width; the field fills its parent when unset.
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
        let filled = self.variant == TextFieldVariant::Filled;
        let focused = self.edit.focused;
        let populated = focused || !self.edit.text.is_empty();

        let input_color = if self.disabled {
            c.on_surface.alpha(0.38)
        } else {
            c.on_surface
        };
        let placeholder_color = if self.disabled {
            c.on_surface.alpha(0.38)
        } else {
            c.on_surface_variant
        };
        let label_color = if self.disabled {
            c.on_surface.alpha(0.38)
        } else if self.error {
            c.error
        } else if focused {
            c.primary
        } else {
            c.on_surface_variant
        };
        let icon_color = if self.disabled {
            c.on_surface.alpha(0.38)
        } else {
            c.on_surface_variant
        };
        let trailing_color = if self.disabled {
            c.on_surface.alpha(0.38)
        } else if self.error {
            c.error
        } else {
            c.on_surface_variant
        };

        let label_x = if self.leading_icon.is_some() {
            52.0
        } else {
            16.0
        };
        let (left_pad, right_pad) = (
            label_x,
            if self.trailing_icon.is_some() {
                52.0
            } else {
                16.0
            },
        );

        let mut field = text_input(&self.edit, theme.type_scale.body_large)
            .single_line()
            .h(56.0)
            .pl(left_pad)
            .pr(right_pad)
            .color(input_color)
            .placeholder(self.placeholder.clone(), placeholder_color)
            .caret(
                if self.error { c.error } else { c.primary },
                c.primary.alpha(0.24),
            )
            .label(self.label.clone())
            .disabled(self.disabled)
            .no_focus_ring();

        field = match self.width {
            Some(width) => field.w(width),
            None => field.w_full(),
        };
        if let Some(key) = self.key {
            field = field.key(key);
        }

        if filled {
            let fill = if self.disabled {
                c.on_surface.alpha(0.04)
            } else {
                c.surface_container_highest
            };
            field = field
                .corners(theme.shape.corner_extra_small_top())
                .fill(fill)
                .pt(24.0)
                .pb(8.0);
        } else {
            let border = if self.disabled {
                c.on_surface.alpha(0.12)
            } else if self.error {
                c.error
            } else if focused {
                c.primary
            } else {
                c.outline
            };
            let width = if focused { 2.0 } else { 1.0 };
            field = field
                .radius(theme.shape.corner_extra_small)
                .border(width, border)
                .pt(16.0)
                .pb(16.0);
        }

        if !self.disabled {
            field = field
                .hover(state_layer(c.on_surface, theme.state.hover))
                .press(state_layer(c.on_surface, theme.state.pressed));
        }

        if filled {
            let indicator = if self.disabled {
                c.on_surface.alpha(0.38)
            } else if self.error {
                c.error
            } else if focused {
                c.primary
            } else {
                c.on_surface_variant
            };
            field = field.child(
                el().key("active-indicator")
                    .a11y_hidden()
                    .pin(Some(0.0), None, Some(0.0), Some(0.0))
                    .h(if focused { 2.0 } else { 1.0 })
                    .fill(indicator),
            );
        }

        let (label_y, label_size, label_line) = if filled {
            if populated {
                (8.0, 12.0, 16.0)
            } else {
                (16.0, 16.0, 24.0)
            }
        } else if populated {
            (-8.0, 12.0, 16.0)
        } else {
            (16.0, 16.0, 24.0)
        };
        let label_style = if populated {
            theme.type_scale.body_small
        } else {
            theme.type_scale.body_large
        };
        let label_style = rune::TextStyle {
            size: label_size,
            line_height: label_line,
            ..label_style
        };
        let mut label = text(self.label.clone())
            .key("label")
            .a11y_hidden()
            .font(label_style)
            .color(label_color);
        if !filled && populated {
            // Material notches the outline around a floating label. Rune has no
            // border notch, so a padded surface fill reproduces the gap.
            label = label
                .pin(Some(label_x - 4.0), Some(label_y), None, None)
                .pl(4.0)
                .pr(4.0)
                .fill(c.surface);
        } else {
            label = label.absolute(label_x, label_y);
        }
        field = field.child(label);

        if let Some(icon) = self.leading_icon {
            field = field.child(
                rune::icon(icon, 24.0)
                    .key("leading-icon")
                    .a11y_hidden()
                    .pin(Some(12.0), Some(16.0), None, None)
                    .color(icon_color),
            );
        }
        if let Some(icon) = self.trailing_icon {
            field = field.child(
                rune::icon(icon, 24.0)
                    .key("trailing-icon")
                    .a11y_hidden()
                    .pin(None, Some(16.0), Some(12.0), None)
                    .color(trailing_color),
            );
        }

        if let Some(supporting) = self.supporting_text {
            let support_color = if self.disabled {
                c.on_surface.alpha(0.38)
            } else if self.error {
                c.error
            } else {
                c.on_surface_variant
            };
            field = field.child(
                text(supporting)
                    .key("supporting-text")
                    .a11y_hidden()
                    .pin(Some(16.0), Some(60.0), Some(16.0), None)
                    .font(theme.type_scale.body_small)
                    .color(support_color),
            );
        }

        field
    }
}
