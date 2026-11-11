//! Menu surfaces and menu items from md.comp.menu and md.comp.menu-item
//! tokens. Menus take caller-built items so press handlers are attached before
//! assembly; the app mounts the menu surface in an overlay layer.

use rune::{El, Icon, col, row, text};

use crate::Theme;
use crate::components::{elevation, state_layer};
use crate::ink::{self, Ink};

/// Builder for a menu item. Selected items use `secondary-container`.
#[derive(Clone, Debug)]
pub struct MenuItem {
    label: String,
    supporting: Option<String>,
    leading_icon: Option<&'static Icon>,
    trailing_text: Option<String>,
    trailing_icon: Option<&'static Icon>,
    selected: bool,
    disabled: bool,
    key: Option<&'static str>,
    ink: Option<Ink>,
}

impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            supporting: None,
            leading_icon: None,
            trailing_text: None,
            trailing_icon: None,
            selected: false,
            disabled: false,
            key: None,
            ink: None,
        }
    }

    pub fn supporting(mut self, supporting: impl Into<String>) -> Self {
        self.supporting = Some(supporting.into());
        self
    }

    pub fn leading_icon(mut self, icon: &'static Icon) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
        self
    }

    pub fn trailing_icon(mut self, icon: &'static Icon) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
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
        let height = if self.supporting.is_some() {
            72.0
        } else {
            56.0
        };

        let (label_color, supporting_color, icon_color) = if self.disabled {
            (
                c.on_surface.alpha(0.38),
                c.on_surface.alpha(0.38),
                c.on_surface.alpha(0.38),
            )
        } else if self.selected {
            (
                c.on_secondary_container,
                c.on_secondary_container,
                c.on_secondary_container,
            )
        } else {
            (c.on_surface, c.on_surface_variant, c.on_surface_variant)
        };

        let mut item = row()
            .w_full()
            .min_h(height)
            .px(16.0)
            .py(12.0)
            .gap(16.0)
            .center_y()
            .role("menuitem")
            .label(self.label.clone())
            .focusable(!self.disabled)
            .disabled(self.disabled);

        if let Some(key) = self.key {
            item = item.key(key);
        }
        if self.selected {
            item = item.fill(c.secondary_container);
        }

        item = ink::capture(&self.ink, self.key, self.disabled, item);
        // First child paints under the content.
        if !self.disabled {
            let ink_layer = if self.selected {
                c.on_secondary_container
            } else {
                c.on_surface
            };
            if let Some(wash) = ink::wash(
                &self.ink,
                self.key,
                state_layer(ink_layer, theme.state.pressed),
                theme.shape.corner_full,
                Some([0.0; 4]),
            ) {
                item = item.child(wash);
            }
        }

        if let Some(icon) = self.leading_icon {
            let mut leading_el = rune::icon(icon, 24.0).color(icon_color);
            if let Some(key) = self.key {
                leading_el = leading_el.key(format!("{key}:leading"));
            }
            item = item.child(leading_el);
        }

        let mut content = col().grow(1.0);
        if let Some(key) = self.key {
            content = content.key(format!("{key}:content"));
        }
        content = content.child(
            text(self.label)
                .font(theme.type_scale.body_large)
                .color(label_color),
        );
        if let Some(supporting) = self.supporting {
            content = content.child(
                text(supporting)
                    .font(theme.type_scale.body_medium)
                    .color(supporting_color),
            );
        }
        item = item.child(content);

        if let Some(trailing) = self.trailing_text {
            let mut trailing_el =
                text(trailing)
                    .font(theme.type_scale.label_small)
                    .color(if self.selected {
                        label_color
                    } else {
                        c.on_surface_variant
                    });
            if let Some(key) = self.key {
                trailing_el = trailing_el.key(format!("{key}:trailing-text"));
            }
            item = item.child(trailing_el);
        }
        if let Some(icon) = self.trailing_icon {
            let mut trailing_el = rune::icon(icon, 24.0).color(icon_color);
            if let Some(key) = self.key {
                trailing_el = trailing_el.key(format!("{key}:trailing"));
            }
            item = item.child(trailing_el);
        }

        if self.disabled {
            item = item
                .hover(rune::Color::TRANSPARENT)
                .press(rune::Color::TRANSPARENT)
                .no_focus_ring();
        } else {
            let layer = if self.selected {
                c.on_secondary_container
            } else {
                c.on_surface
            };
            item = item
                .hover(state_layer(layer, theme.state.hover))
                .press(state_layer(layer, theme.state.pressed));
        }
        item
    }
}

/// Builder for a menu surface: surface-container, level 2 and 4 pt corners.
#[derive(Clone)]
pub struct Menu {
    children: Vec<El>,
    min_width: f32,
    width: Option<f32>,
    key: Option<&'static str>,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            min_width: 112.0,
            width: None,
            key: None,
        }
    }
}

impl Menu {
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

    pub fn min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let mut menu = col()
            .min_w(self.min_width)
            .py(8.0)
            .radius(theme.shape.corner_extra_small)
            .fill(theme.color.surface_container)
            .role("menu");
        if let Some(width) = self.width {
            menu = menu.w(width);
        }
        if let Some(key) = self.key {
            menu = menu.key(key);
        }
        menu = elevation(menu, theme, 2);
        menu.children(self.children)
    }
}
