//! List items and lists from md.comp.list tokens. Lists take caller-built
//! items so press handlers are attached before assembly.

use rune::{El, Icon, col, row, text};

use crate::Theme;
use crate::components::{Divider, state_layer};
use crate::ink::{self, Ink};

/// Builder for a list item: 56 pt for one line, 72 pt for two and 88 pt for
/// three, with 16 pt leading and trailing space.
#[derive(Clone, Debug)]
pub struct ListItem {
    overline: Option<String>,
    headline: String,
    supporting: Option<String>,
    leading_icon: Option<&'static Icon>,
    trailing_text: Option<String>,
    trailing_icon: Option<&'static Icon>,
    selected: bool,
    disabled: bool,
    key: Option<&'static str>,
    ink: Option<Ink>,
}

impl ListItem {
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            overline: None,
            headline: headline.into(),
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

    pub fn overline(mut self, overline: impl Into<String>) -> Self {
        self.overline = Some(overline.into());
        self
    }

    pub fn supporting(mut self, supporting: impl Into<String>) -> Self {
        self.supporting = Some(supporting.into());
        self
    }

    /// A 24 pt leading icon.
    pub fn leading_icon(mut self, icon: &'static Icon) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    /// A trailing label-small value, such as a count.
    pub fn trailing_text(mut self, text: impl Into<String>) -> Self {
        self.trailing_text = Some(text.into());
        self
    }

    /// A 24 pt trailing icon. Selection turns it primary.
    pub fn trailing_icon(mut self, icon: &'static Icon) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    /// Selection only colours the trailing icon; the app owns any container.
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
        let lines =
            1 + usize::from(self.overline.is_some()) + usize::from(self.supporting.is_some());
        let height = match lines {
            1 => 56.0,
            2 => 72.0,
            _ => 88.0,
        };

        let (label_color, supporting_color, overline_color, icon_color) = if self.disabled {
            (
                c.on_surface.alpha(0.38),
                c.on_surface.alpha(0.38),
                c.on_surface.alpha(0.38),
                c.on_surface.alpha(0.38),
            )
        } else {
            (
                c.on_surface,
                c.on_surface_variant,
                c.on_surface_variant,
                c.on_surface_variant,
            )
        };
        let trailing_color = if self.disabled {
            c.on_surface.alpha(0.38)
        } else if self.selected {
            c.primary
        } else {
            c.on_surface_variant
        };

        let mut item = row()
            .w_full()
            .min_h(height)
            .px(16.0)
            .py(12.0)
            .gap(16.0)
            .center_y();

        if let Some(key) = self.key {
            item = item.key(key);
        }

        item = ink::capture(&self.ink, self.key, self.disabled, item);
        // First child paints under the content.
        if !self.disabled
            && let Some(wash) = ink::wash(
                &self.ink,
                self.key,
                state_layer(c.on_surface, theme.state.pressed),
                theme.shape.corner_full,
                Some([0.0; 4]),
            )
        {
            item = item.child(wash);
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
        if let Some(overline) = self.overline {
            content = content.child(
                text(overline)
                    .font(theme.type_scale.label_small)
                    .color(overline_color),
            );
        }
        content = content.child(
            text(self.headline)
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
            let mut trailing_el = text(trailing)
                .font(theme.type_scale.label_small)
                .color(c.on_surface_variant);
            if let Some(key) = self.key {
                trailing_el = trailing_el.key(format!("{key}:trailing-text"));
            }
            item = item.child(trailing_el);
        }
        if let Some(icon) = self.trailing_icon {
            let mut trailing_el = rune::icon(icon, 24.0).color(trailing_color);
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
            item = item
                .hover(state_layer(c.on_surface, theme.state.hover))
                .press(state_layer(c.on_surface, theme.state.pressed));
        }
        item
    }
}

/// Builder for a full-width column of list items, optionally separated by
/// 16 pt inset dividers.
#[derive(Clone, Default)]
pub struct List {
    children: Vec<El>,
    dividers: bool,
    key: Option<&'static str>,
}

impl List {
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

    pub fn dividers(mut self, dividers: bool) -> Self {
        self.dividers = dividers;
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let mut list = col().w_full().fill(theme.color.surface);
        if let Some(key) = self.key {
            list = list.key(key);
        }
        let last = self.children.len().saturating_sub(1);
        for (i, child) in self.children.into_iter().enumerate() {
            list = list.child(child);
            if self.dividers && i < last {
                list = list.child(Divider::new().inset(16.0).build(theme));
            }
        }
        list
    }
}
