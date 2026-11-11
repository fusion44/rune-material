//! Filled and outlined selects from md.comp.*-select tokens. The field is
//! field-rooted; the app owns the menu overlay and attaches `.on_press(..)` to
//! open it.

use rune::{El, el, row, text};

use crate::Theme;
use crate::components::{icons, state_layer};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SelectVariant {
    #[default]
    Filled,
    Outlined,
}

/// Builder for a Material 3 select field. Minimum width is 210 pt.
#[derive(Clone, Debug)]
pub struct Select {
    label: String,
    value: String,
    variant: SelectVariant,
    supporting_text: Option<String>,
    error: bool,
    disabled: bool,
    open: bool,
    width: Option<f32>,
    key: Option<&'static str>,
}

impl Select {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: String::new(),
            variant: SelectVariant::Filled,
            supporting_text: None,
            error: false,
            disabled: false,
            open: false,
            width: None,
            key: None,
        }
    }

    /// The selected item's label. An empty value leaves the label resting.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn variant(mut self, variant: SelectVariant) -> Self {
        self.variant = variant;
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

    /// Draw the open state: primary outline/indicator, as the menu is showing.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Field width. Selects are at least 210 pt wide.
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
        let filled = self.variant == SelectVariant::Filled;
        let focused = self.open && !self.disabled;
        let populated = !self.value.is_empty();

        let label_color = if self.disabled {
            c.on_surface.alpha(0.38)
        } else if self.error {
            c.error
        } else if focused {
            c.primary
        } else {
            c.on_surface_variant
        };
        let value_color = if self.disabled {
            c.on_surface.alpha(0.38)
        } else {
            c.on_surface
        };
        let icon_color = if self.disabled {
            c.on_surface.alpha(0.38)
        } else if self.error {
            c.error
        } else {
            c.on_surface_variant
        };

        let mut field = row()
            .role("button")
            .label(self.label.clone())
            .focusable(!self.disabled)
            .disabled(self.disabled)
            .h(56.0)
            .min_w(210.0)
            .pl(16.0)
            .pr(52.0);

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
                .fill(fill);
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
                .border(width, border);
        }

        if !self.disabled {
            field = field
                .hover(state_layer(c.on_surface, theme.state.hover))
                .press(state_layer(c.on_surface, theme.state.pressed))
                .focus_ring(3.0, 2.0, c.secondary);
        }

        let (label_y, label_line) = if filled {
            if populated || focused {
                (8.0, 16.0)
            } else {
                (16.0, 24.0)
            }
        } else if populated || focused {
            (-8.0, 16.0)
        } else {
            (16.0, 24.0)
        };
        let label_style = if populated || focused {
            rune::TextStyle {
                size: 12.0,
                line_height: label_line,
                ..theme.type_scale.body_small
            }
        } else {
            rune::TextStyle {
                size: 16.0,
                line_height: label_line,
                ..theme.type_scale.body_large
            }
        };
        let mut label = text(self.label.clone())
            .key("label")
            .a11y_hidden()
            .font(label_style)
            .color(label_color);
        if !filled && (populated || focused) {
            label = label
                .pin(Some(12.0), Some(label_y), None, None)
                .pl(4.0)
                .pr(4.0)
                .fill(c.surface);
        } else {
            label = label.absolute(16.0, label_y);
        }
        field = field.child(label);

        if !self.value.is_empty() {
            field = field.child(
                text(self.value.clone())
                    .key("value")
                    .a11y_hidden()
                    .absolute(16.0, if filled { 24.0 } else { 16.0 })
                    .font(theme.type_scale.body_large)
                    .color(value_color),
            );
        }

        field = field.child(
            rune::icon(&icons::ARROW_DROP_DOWN, 24.0)
                .key("menu-icon")
                .a11y_hidden()
                .pin(None, Some(16.0), Some(12.0), None)
                .color(icon_color),
        );

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
