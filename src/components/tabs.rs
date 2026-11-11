//! Primary and secondary navigation tabs from md.comp.*-navigation-tab
//! tokens. Tabs take caller-built tab elements so press handlers are attached
//! before assembly.

use crate::Theme;
use crate::components::state_layer;
use crate::ink::{self, Ink};
use rune::{El, Icon, col, el, row, text};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TabVariant {
    #[default]
    Primary,
    Secondary,
}

/// Builder for a single 48 pt navigation tab, or 64 pt with an icon. The app
/// owns selection and attaches `.on_press(..)` to the returned element.
#[derive(Clone, Debug)]
pub struct Tab {
    label: String,
    variant: TabVariant,
    selected: bool,
    icon: Option<&'static Icon>,
    disabled: bool,
    key: Option<&'static str>,
    ink: Option<Ink>,
}

impl Tab {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: TabVariant::Primary,
            selected: false,
            icon: None,
            disabled: false,
            key: None,
            ink: None,
        }
    }

    pub fn variant(mut self, variant: TabVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn icon(mut self, icon: &'static Icon) -> Self {
        self.icon = Some(icon);
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

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let c = &theme.color;
        let primary = self.variant == TabVariant::Primary;

        let (label_color, icon_color) = if self.disabled {
            (c.on_surface.alpha(0.38), c.on_surface.alpha(0.38))
        } else if primary {
            if self.selected {
                (c.primary, c.primary)
            } else {
                (c.on_surface_variant, c.on_surface_variant)
            }
        } else if self.selected {
            (c.on_surface, c.on_surface)
        } else {
            (c.on_surface_variant, c.on_surface_variant)
        };

        let mut tab = col()
            .role("tab")
            .checked(self.selected)
            .label(self.label.clone())
            .focusable(!self.disabled)
            .disabled(self.disabled)
            .h(if self.icon.is_some() { 64.0 } else { 48.0 })
            .px(16.0)
            .center();

        if let Some(key) = self.key {
            tab = tab.key(key);
        }

        let layer = if primary {
            if self.selected {
                c.primary
            } else {
                c.on_surface
            }
        } else {
            c.on_surface
        };

        tab = ink::capture(&self.ink, self.key, self.disabled, tab);
        // First child paints under the label, icon and indicator.
        if !self.disabled
            && let Some(wash) = ink::wash(
                &self.ink,
                self.key,
                state_layer(layer, theme.state.pressed),
                theme.shape.corner_full,
                Some([0.0; 4]),
            )
        {
            tab = tab.child(wash);
        }

        if let Some(icon) = self.icon {
            let mut icon_el = rune::icon(icon, 24.0).color(icon_color);
            if let Some(key) = self.key {
                icon_el = icon_el.key(format!("{key}:icon"));
            }
            tab = tab.child(icon_el);
        }
        let mut label_el = text(self.label)
            .font(theme.type_scale.title_small)
            .color(label_color);
        if let Some(key) = self.key {
            label_el = label_el.key(format!("{key}:label"));
        }
        tab = tab.child(label_el);

        if self.disabled {
            tab = tab
                .hover(rune::Color::TRANSPARENT)
                .press(rune::Color::TRANSPARENT)
                .no_focus_ring();
        } else {
            tab = tab
                .hover(state_layer(layer, theme.state.hover))
                .press(state_layer(layer, theme.state.pressed))
                .focus_ring(3.0, 2.0, c.secondary);
        }

        if self.selected {
            let (height, corners) = if primary {
                (3.0, [3.0, 3.0, 0.0, 0.0])
            } else {
                (2.0, [0.0; 4])
            };
            tab = tab.child(
                el().key("active-indicator")
                    .a11y_hidden()
                    .pin(Some(0.0), None, Some(0.0), Some(0.0))
                    .h(height)
                    .corners(corners)
                    .fill(c.primary),
            );
        }

        tab
    }
}

/// Builder for the row that groups tabs. Tabs share the width equally.
#[derive(Clone)]
pub struct Tabs {
    children: Vec<El>,
    equal_width: bool,
    key: Option<&'static str>,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            equal_width: true,
            key: None,
        }
    }
}

impl Tabs {
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

    pub fn equal_width(mut self, equal_width: bool) -> Self {
        self.equal_width = equal_width;
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, _theme: &Theme) -> El {
        let mut tabs = row().w_full().items_stretch();
        if let Some(key) = self.key {
            tabs = tabs.key(key);
        }
        for child in self.children {
            tabs = tabs.child(if self.equal_width {
                child.grow(1.0)
            } else {
                child
            });
        }
        tabs
    }
}
