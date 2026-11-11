//! Filled, elevated, tonal, outlined and text buttons from md.comp.*-button tokens.

use rune::{Color, El, Icon, row, text};

use crate::Theme;
use crate::components::{elevation, state_layer};
use crate::ink::{self, Ink};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Filled,
    Elevated,
    Tonal,
    Outlined,
    Text,
}

/// Builder for a Material 3 common button.
///
/// ```ignore
/// row().child(
///     Button::new("Save")
///         .key("save")
///         .variant(ButtonVariant::Filled)
///         .icon(&SAVE_ICON)
///         .build(&theme)
///         .on_press(|s: &mut State| s.save()),
/// );
/// ```
#[derive(Clone, Debug)]
pub struct Button {
    label: String,
    variant: ButtonVariant,
    icon: Option<&'static Icon>,
    key: Option<&'static str>,
    disabled: bool,
    ink: Option<Ink>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Filled,
            icon: None,
            key: None,
            disabled: false,
            ink: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// A leading icon, 18 pt; leading space becomes 16 pt as in Material Web.
    pub fn icon(mut self, icon: &'static Icon) -> Self {
        self.icon = Some(icon);
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

    /// Material ink ripple on pointer press, painted from the app's [`Ink`]
    /// store (see `rune_material::ink`). Needs `.key(..)`; occupies the
    /// element's `on_drag` slot for origin capture.
    pub fn ink(mut self, ink: &Ink) -> Self {
        self.ink = Some(ink.clone());
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let c = &theme.color;
        let (fill, label_color, icon_color, layer_color, level) = match self.variant {
            ButtonVariant::Filled => (Some(c.primary), c.on_primary, c.on_primary, c.on_primary, 0),
            ButtonVariant::Elevated => (
                Some(c.surface_container_low),
                c.primary,
                c.primary,
                c.primary,
                1,
            ),
            ButtonVariant::Tonal => (
                Some(c.secondary_container),
                c.on_secondary_container,
                c.on_secondary_container,
                c.on_secondary_container,
                0,
            ),
            ButtonVariant::Outlined => (None, c.primary, c.primary, c.primary, 0),
            ButtonVariant::Text => (None, c.primary, c.primary, c.primary, 0),
        };

        let mut container = row()
            .role("button")
            .label(self.label.clone())
            .focusable(!self.disabled)
            .disabled(self.disabled)
            .h(40.0)
            .radius(theme.shape.corner_full)
            .center_y()
            .gap(theme.spacing.x2);

        if let Some(key) = self.key {
            container = container.key(key);
        }

        if self.icon.is_some() {
            container = container.pl(16.0).pr(24.0);
        } else {
            container = container.pl(24.0).pr(24.0);
        }

        let mut fill_color = fill;
        let mut label_color = label_color;
        let mut icon_color = icon_color;
        let outline = self.variant == ButtonVariant::Outlined;

        if self.disabled {
            if self.variant != ButtonVariant::Text {
                fill_color = Some(c.on_surface.alpha(0.12));
            }
            label_color = c.on_surface.alpha(0.38);
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
            let wash = ink::wash(
                &self.ink,
                self.key,
                state_layer(layer_color, theme.state.pressed),
                theme.shape.corner_full,
                Some([theme.shape.corner_full; 4]),
            );
            if let Some(wash) = wash {
                container = container.child(wash);
            }
        }

        let mut content = row().center_y().gap(theme.spacing.x2);
        if let Some(key) = self.key {
            content = content.key(format!("{key}:content"));
        }
        if let Some(icon) = self.icon {
            content = content.child(rune::icon(icon, 18.0).color(icon_color));
        }
        content = content.child(
            text(self.label)
                .font(theme.type_scale.label_large)
                .color(label_color),
        );
        container = container.child(content);

        if !self.disabled {
            container = elevation(container, theme, level);
        }

        container
    }
}
