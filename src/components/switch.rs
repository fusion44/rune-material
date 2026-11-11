//! Switch from md.comp.switch tokens: 52 x 32 pt track, 40 pt state layer,
//! 24 pt selected handle, 16 pt unselected handle, 16 pt optional icons.

use rune::{Color, El, Spring, el};

use crate::Theme;
use crate::components::{icons, state_layer};

/// Builder for a Material 3 switch. The caller owns the selected state and
/// attaches `.on_press(...)` to the returned `El`.
#[derive(Clone, Debug)]
pub struct Switch {
    selected: bool,
    label: String,
    icons: bool,
    key: Option<&'static str>,
    disabled: bool,
}

impl Switch {
    pub fn new(selected: bool, label: impl Into<String>) -> Self {
        Self {
            selected,
            label: label.into(),
            icons: false,
            key: None,
            disabled: false,
        }
    }

    /// Show the selected check icon and the unselected close icon.
    pub fn icons(mut self, icons: bool) -> Self {
        self.icons = icons;
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
        let full = theme.shape.corner_full;

        let (track, track_outline) = if self.disabled {
            if self.selected {
                (Some(c.on_surface.alpha(0.12)), None)
            } else {
                (
                    Some(c.surface_container_highest.alpha(0.12)),
                    Some(c.on_surface.alpha(0.12)),
                )
            }
        } else if self.selected {
            (Some(c.primary), None)
        } else {
            (Some(c.surface_container_highest), Some(c.outline))
        };

        let handle_color = if self.disabled {
            if self.selected {
                c.surface
            } else {
                c.on_surface.alpha(0.38)
            }
        } else if self.selected {
            c.on_primary
        } else {
            c.outline
        };

        let mut root = el()
            .role("switch")
            .label(self.label.clone())
            .checked(self.selected)
            .focusable(!self.disabled)
            .disabled(self.disabled)
            .size(52.0, 32.0)
            .radius(full);

        if let Some(key) = self.key {
            root = root.key(key);
        }

        if let Some(fill) = track {
            root = root.fill(fill);
        }
        if let Some(outline) = track_outline {
            root = root.border(2.0, outline);
        }

        let handle_size = if self.icons || self.selected {
            24.0
        } else {
            16.0
        };
        let handle_x = if self.selected {
            24.0
        } else if self.icons {
            4.0
        } else {
            8.0
        };
        let handle_y = (32.0 - handle_size) / 2.0;

        let mut handle = el()
            .key("handle")
            .a11y_hidden()
            .size(handle_size, handle_size)
            .absolute(handle_x, handle_y)
            .radius(full)
            .fill(handle_color)
            .motion(Spring::new(theme.motion.medium2.as_secs_f32(), 0.2));

        if self.icons {
            let (glyph, color) = if self.selected {
                (
                    &icons::CHECK,
                    if self.disabled {
                        c.on_surface.alpha(0.38)
                    } else {
                        c.on_primary_container
                    },
                )
            } else {
                (
                    &icons::CLOSE,
                    if self.disabled {
                        c.surface_container_highest.alpha(0.38)
                    } else {
                        c.surface_container_highest
                    },
                )
            };
            handle = handle.center().child(rune::icon(glyph, 16.0).color(color));
        }

        root = root.child(handle);

        let layer = if self.disabled {
            Color::TRANSPARENT
        } else if self.selected {
            c.primary
        } else {
            c.on_surface
        };

        if self.disabled {
            root = root
                .hover(Color::TRANSPARENT)
                .press(Color::TRANSPARENT)
                .no_focus_ring();
        } else {
            root = root
                .hover(state_layer(layer, theme.state.hover))
                .press(state_layer(layer, theme.state.pressed))
                .focus_ring(2.0, 1.0, c.secondary);
        }

        root
    }
}
