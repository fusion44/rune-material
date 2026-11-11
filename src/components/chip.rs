//! Assist, filter, input and suggestion chips from md.comp.*-chip tokens, and
//! the 8 pt chip set that groups them.

use rune::{Color, El, Icon, row, text};

use crate::Theme;
use crate::components::{elevation, state_layer};
use crate::ink::{self, Ink};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChipVariant {
    #[default]
    Assist,
    Filter,
    Input,
    Suggestion,
}

/// Builder for a Material 3 chip. The caller owns selection and attaches
/// `.on_press(..)` to the returned element.
#[derive(Clone, Debug)]
pub struct Chip {
    label: String,
    variant: ChipVariant,
    selected: bool,
    elevated: bool,
    icon: Option<&'static Icon>,
    trailing_icon: Option<&'static Icon>,
    key: Option<&'static str>,
    disabled: bool,
    ink: Option<Ink>,
}

impl Chip {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ChipVariant::Assist,
            selected: false,
            elevated: false,
            icon: None,
            trailing_icon: None,
            key: None,
            disabled: false,
            ink: None,
        }
    }

    pub fn variant(mut self, variant: ChipVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Selection state for filter and input chips.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// The elevated form (assist, filter and suggestion chips).
    pub fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }

    /// A leading 18 pt icon.
    pub fn icon(mut self, icon: &'static Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// A trailing 18 pt icon, such as an input chip's remove glyph.
    pub fn trailing_icon(mut self, icon: &'static Icon) -> Self {
        self.trailing_icon = Some(icon);
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

        let (fill, outline, label_color, leading_color, trailing_color, layer, level) =
            match self.variant {
                ChipVariant::Assist => {
                    let fill = self.elevated.then_some(c.surface_container_low);
                    (
                        fill,
                        (!self.elevated).then_some(c.outline),
                        c.on_surface,
                        c.primary,
                        c.on_surface_variant,
                        c.on_surface,
                        if self.elevated { 1 } else { 0 },
                    )
                }
                ChipVariant::Suggestion => {
                    let fill = self.elevated.then_some(c.surface_container_low);
                    (
                        fill,
                        (!self.elevated).then_some(c.outline),
                        c.on_surface_variant,
                        c.primary,
                        c.on_surface_variant,
                        c.on_surface_variant,
                        if self.elevated { 1 } else { 0 },
                    )
                }
                ChipVariant::Filter => {
                    if self.selected {
                        (
                            Some(c.secondary_container),
                            None,
                            c.on_secondary_container,
                            c.on_secondary_container,
                            c.on_secondary_container,
                            c.on_secondary_container,
                            if self.elevated { 1 } else { 0 },
                        )
                    } else if self.elevated {
                        (
                            Some(c.surface_container_low),
                            None,
                            c.on_surface_variant,
                            c.primary,
                            c.on_surface_variant,
                            c.on_surface_variant,
                            1,
                        )
                    } else {
                        (
                            None,
                            Some(c.outline),
                            c.on_surface_variant,
                            c.primary,
                            c.on_surface_variant,
                            c.on_surface_variant,
                            0,
                        )
                    }
                }
                ChipVariant::Input => {
                    if self.selected {
                        (
                            Some(c.secondary_container),
                            None,
                            c.on_secondary_container,
                            c.primary,
                            c.on_secondary_container,
                            c.on_secondary_container,
                            0,
                        )
                    } else {
                        (
                            None,
                            Some(c.outline),
                            c.on_surface_variant,
                            c.on_surface_variant,
                            c.on_surface_variant,
                            c.on_surface_variant,
                            0,
                        )
                    }
                }
            };

        let selectable = matches!(self.variant, ChipVariant::Filter | ChipVariant::Input);
        let mut root = row()
            .role(if selectable { "checkbox" } else { "button" })
            .label(self.label.clone())
            .focusable(!self.disabled)
            .disabled(self.disabled)
            .h(32.0)
            .radius(theme.shape.corner_small)
            .center_y()
            .gap(8.0);

        if selectable {
            root = root.checked(self.selected);
        }
        if let Some(key) = self.key {
            root = root.key(key);
        }

        let left = if self.icon.is_some() { 8.0 } else { 16.0 };
        let right = if self.trailing_icon.is_some() {
            8.0
        } else {
            16.0
        };
        root = root.pl(left).pr(right);

        let (fill, outline, label_color, leading_color, trailing_color) = if self.disabled {
            (
                if self.elevated || (selectable && self.selected) {
                    Some(c.on_surface.alpha(0.12))
                } else {
                    fill
                },
                if selectable && !self.selected {
                    Some(c.on_surface.alpha(0.12))
                } else {
                    outline
                },
                c.on_surface.alpha(0.38),
                c.on_surface.alpha(0.38),
                c.on_surface.alpha(0.38),
            )
        } else {
            (fill, outline, label_color, leading_color, trailing_color)
        };

        if let Some(fill) = fill {
            root = root.fill(fill);
        }
        if let Some(outline) = outline {
            root = root.border(1.0, outline);
        }

        if self.disabled {
            root = root
                .hover(Color::TRANSPARENT)
                .press(Color::TRANSPARENT)
                .no_focus_ring();
        } else {
            root = root
                .hover(state_layer(layer, theme.state.hover))
                .press(state_layer(layer, theme.state.pressed))
                .focus_ring(3.0, 2.0, c.secondary);
        }

        root = ink::capture(&self.ink, self.key, self.disabled, root);
        // First child paints under the label and icons.
        if !self.disabled
            && let Some(wash) = ink::wash(
                &self.ink,
                self.key,
                state_layer(layer, theme.state.pressed),
                theme.shape.corner_full,
                Some([theme.shape.corner_small; 4]),
            )
        {
            root = root.child(wash);
        }

        if let Some(icon) = self.icon {
            let mut icon_el = rune::icon(icon, 18.0).color(leading_color);
            if let Some(key) = self.key {
                icon_el = icon_el.key(format!("{key}:icon"));
            }
            root = root.child(icon_el);
        }
        let mut label_el = text(self.label)
            .font(theme.type_scale.label_large)
            .color(label_color);
        if let Some(key) = self.key {
            label_el = label_el.key(format!("{key}:label"));
        }
        root = root.child(label_el);
        if let Some(icon) = self.trailing_icon {
            let mut trailing_el = rune::icon(icon, 18.0).color(trailing_color);
            if let Some(key) = self.key {
                trailing_el = trailing_el.key(format!("{key}:trailing"));
            }
            root = root.child(trailing_el);
        }

        if !self.disabled {
            root = elevation(root, theme, level);
        }
        root
    }
}

/// Builder for the 8 pt gap row that groups chips, wrapping onto new lines.
#[derive(Clone, Default)]
pub struct ChipSet {
    children: Vec<El>,
    key: Option<&'static str>,
}

impl ChipSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn child(mut self, child: El) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = El>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, _theme: &Theme) -> El {
        let mut set = row().gap(8.0).center_y().flex_wrap();
        if let Some(key) = self.key {
            set = set.key(key);
        }
        for child in self.children {
            set = set.child(child);
        }
        set
    }
}
