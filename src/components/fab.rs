//! Primary, secondary, tertiary and surface floating action buttons, with regular,
//! small and large sizes, from md.comp.fab-* tokens.

use rune::{Color, El, Icon, row};

use crate::Theme;
use crate::components::{elevation, state_layer};
use crate::ink::{self, Ink};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FabVariant {
    #[default]
    Primary,
    Secondary,
    Tertiary,
    Surface,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FabSize {
    #[default]
    Regular,
    Small,
    Large,
}

/// Builder for a Material 3 floating action button.
#[derive(Clone, Debug)]
pub struct Fab {
    icon: &'static Icon,
    label: String,
    variant: FabVariant,
    size: FabSize,
    lowered: bool,
    key: Option<&'static str>,
    disabled: bool,
    ink: Option<Ink>,
}

impl Fab {
    pub fn new(icon: &'static Icon, label: impl Into<String>) -> Self {
        Self {
            icon,
            label: label.into(),
            variant: FabVariant::Primary,
            size: FabSize::Regular,
            lowered: false,
            key: None,
            disabled: false,
            ink: None,
        }
    }

    pub fn variant(mut self, variant: FabVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: FabSize) -> Self {
        self.size = size;
        self
    }

    /// Surface FABs have a lowered form (md.comp.fab-surface.lowered-*).
    pub fn lowered(mut self, lowered: bool) -> Self {
        self.lowered = lowered;
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

        let (dim, radius, icon_size) = match self.size {
            FabSize::Regular => (56.0, theme.shape.corner_large, 24.0),
            FabSize::Small => (40.0, theme.shape.corner_medium, 24.0),
            FabSize::Large => (96.0, theme.shape.corner_extra_large, 36.0),
        };

        let (fill, icon_color, layer_color) = match self.variant {
            FabVariant::Primary => (
                c.primary_container,
                c.on_primary_container,
                c.on_primary_container,
            ),
            FabVariant::Secondary => (
                c.secondary_container,
                c.on_secondary_container,
                c.on_secondary_container,
            ),
            FabVariant::Tertiary => (
                c.tertiary_container,
                c.on_tertiary_container,
                c.on_tertiary_container,
            ),
            FabVariant::Surface => (c.surface_container_high, c.primary, c.primary),
        };

        let mut container = row()
            .role("button")
            .label(self.label.clone())
            .focusable(!self.disabled)
            .disabled(self.disabled)
            .size(dim, dim)
            .radius(radius)
            .center();

        if let Some(key) = self.key {
            container = container.key(key);
        }

        let mut fill_color = fill;
        let mut icon_color = icon_color;
        let mut level = 3;
        let lowered = self.lowered && self.variant == FabVariant::Surface;
        if lowered {
            fill_color = c.surface_container_low;
            level = 1;
        }

        if self.disabled {
            fill_color = c.on_surface.alpha(0.12);
            icon_color = c.on_surface.alpha(0.38);
            container = container.fill(fill_color);
            container = container
                .hover(Color::TRANSPARENT)
                .press(Color::TRANSPARENT)
                .no_focus_ring();
        } else {
            container = container.fill(fill_color);
            container = container
                .hover(state_layer(layer_color, theme.state.hover))
                .press(state_layer(layer_color, theme.state.pressed))
                .focus_ring(3.0, 2.0, c.secondary);
            container = elevation(container, theme, level);
        }

        container = ink::capture(&self.ink, self.key, self.disabled, container);
        if !self.disabled
            && let Some(wash) = ink::wash(
                &self.ink,
                self.key,
                state_layer(layer_color, theme.state.pressed),
                theme.shape.corner_full,
                Some([radius; 4]),
            )
        {
            container = container.child(wash);
        }

        let mut icon_el = rune::icon(self.icon, icon_size).color(icon_color);
        if let Some(key) = self.key {
            icon_el = icon_el.key(format!("{key}:icon"));
        }
        container = container.child(icon_el);
        container
    }
}
