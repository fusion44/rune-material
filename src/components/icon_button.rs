//! Filled, tonal and outlined icon buttons from md.comp.*-icon-button tokens.

use rune::{Color, El, Icon, row};

use crate::Theme;
use crate::components::state_layer;
use crate::ink::{self, Ink};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum IconButtonVariant {
    #[default]
    Filled,
    Tonal,
    Outlined,
}

/// Builder for a Material 3 icon button. `selected(Some(..))` makes it a toggle button.
#[derive(Clone, Debug)]
pub struct IconButton {
    icon: &'static Icon,
    label: String,
    variant: IconButtonVariant,
    selected: Option<bool>,
    key: Option<&'static str>,
    disabled: bool,
    ink: Option<Ink>,
}

impl IconButton {
    pub fn new(icon: &'static Icon, label: impl Into<String>) -> Self {
        Self {
            icon,
            label: label.into(),
            variant: IconButtonVariant::Filled,
            selected: None,
            key: None,
            disabled: false,
            ink: None,
        }
    }

    pub fn variant(mut self, variant: IconButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn selected(mut self, selected: Option<bool>) -> Self {
        self.selected = selected;
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

    /// Material ink ripple on pointer press (see `rune_material::ink`). Needs
    /// `.key(..)`; occupies the element's `on_drag` slot for origin capture.
    pub fn ink(mut self, ink: &Ink) -> Self {
        self.ink = Some(ink.clone());
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let c = &theme.color;

        let (fill, icon_color, layer_color, outline_color) = match (self.variant, self.selected) {
            (IconButtonVariant::Filled, Some(false)) => (
                Some(c.surface_container_highest),
                c.primary,
                c.primary,
                None,
            ),
            (IconButtonVariant::Filled, _) => (Some(c.primary), c.on_primary, c.on_primary, None),
            (IconButtonVariant::Tonal, Some(false)) => (
                Some(c.surface_container_highest),
                c.on_surface_variant,
                c.on_surface_variant,
                None,
            ),
            (IconButtonVariant::Tonal, _) => (
                Some(c.secondary_container),
                c.on_secondary_container,
                c.on_secondary_container,
                None,
            ),
            (IconButtonVariant::Outlined, Some(true)) => (
                Some(c.inverse_surface),
                c.inverse_on_surface,
                c.inverse_on_surface,
                None,
            ),
            (IconButtonVariant::Outlined, _) => (
                None,
                c.on_surface_variant,
                c.on_surface_variant,
                Some(c.outline),
            ),
        };

        let mut container = row()
            .role("button")
            .label(self.label.clone())
            .focusable(!self.disabled)
            .disabled(self.disabled)
            .size(40.0, 40.0)
            .radius(theme.shape.corner_full)
            .center();

        if let Some(key) = self.key {
            container = container.key(key);
        }
        if let Some(selected) = self.selected {
            container = container.checked(selected);
        }

        let mut fill_color = fill;
        let mut icon_color = icon_color;
        let outline = outline_color.is_some();

        if self.disabled {
            if self.variant != IconButtonVariant::Outlined || self.selected == Some(true) {
                fill_color = Some(c.on_surface.alpha(0.12));
            }
            icon_color = c.on_surface.alpha(0.38);
            if outline {
                container = container.border(1.0, c.on_surface.alpha(0.12));
            }
            container = container
                .hover(Color::TRANSPARENT)
                .press(Color::TRANSPARENT)
                .no_focus_ring();
        } else {
            if outline {
                container = container.border(1.0, c.outline);
            }
            container = container
                .hover(state_layer(layer_color, theme.state.hover))
                .press(state_layer(layer_color, theme.state.pressed))
                .focus_ring(3.0, 2.0, c.secondary);
        }

        if let Some(fill) = fill_color {
            container = container.fill(fill);
        }

        container = ink::capture(&self.ink, self.key, self.disabled, container);
        if !self.disabled {
            // Clip only the ink, not the button: focus rings may still sit
            // outside the control, but ripples stay inside its circle.
            if let Some(wash) = ink::wash(
                &self.ink,
                self.key,
                state_layer(layer_color, theme.state.pressed),
                theme.shape.corner_full,
                Some([theme.shape.corner_full; 4]),
            ) {
                container = container.child(wash);
            }
        }

        let mut icon_el = rune::icon(self.icon, 24.0).color(icon_color);
        if let Some(key) = self.key {
            icon_el = icon_el.key(format!("{key}:icon"));
        }
        container = container.child(icon_el);
        container
    }
}
