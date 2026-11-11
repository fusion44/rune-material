//! Elevated, filled and outlined cards from md.comp.*-card tokens.

use rune::{El, col};

use crate::Theme;
use crate::components::{elevation, state_layer};
use crate::ink::{self, Ink};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CardVariant {
    #[default]
    Elevated,
    Filled,
    Outlined,
}

/// Builder for a Material 3 card surface. Callers supply the content and attach
/// any press handlers to the returned element.
#[derive(Clone, Default)]
pub struct Card {
    variant: CardVariant,
    children: Vec<El>,
    padding: f32,
    width: Option<f32>,
    disabled: bool,
    key: Option<&'static str>,
    ink: Option<Ink>,
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn child(mut self, child: El) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = El>) -> Self {
        self.children.extend(children);
        self
    }

    /// Content padding. Cards are unpadded by default so media can bleed.
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Material ink ripple on pointer press (see `rune_material::ink`). Needs
    /// `.key(..)`; occupies the element's `on_drag` slot for origin capture.
    /// Attach an app `on_press` for the card to actually do something.
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

        let (fill, outline, level) = match self.variant {
            CardVariant::Elevated => (Some(c.surface_container_low), None, 1),
            CardVariant::Filled => (Some(c.surface_container_highest), None, 0),
            CardVariant::Outlined => (Some(c.surface), Some(c.outline_variant), 0),
        };

        let mut card = col()
            .radius(theme.shape.corner_medium)
            .padding(self.padding);
        if let Some(key) = self.key {
            card = card.key(key);
        }
        if let Some(width) = self.width {
            card = card.w(width);
        }

        if self.disabled {
            let (fill, outline) = match self.variant {
                CardVariant::Elevated => (Some(c.surface.alpha(0.38)), outline),
                CardVariant::Filled => (Some(c.surface_variant.alpha(0.38)), outline),
                CardVariant::Outlined => (fill, Some(c.outline.alpha(0.12))),
            };
            if let Some(fill) = fill {
                card = card.fill(fill);
            }
            if let Some(outline) = outline {
                card = card.border(1.0, outline);
            }
            return card.children(self.children);
        }

        if let Some(fill) = fill {
            card = card.fill(fill);
        }
        if let Some(outline) = outline {
            card = card.border(1.0, outline);
        }
        card = elevation(card, theme, level);
        card = ink::capture(&self.ink, self.key, self.disabled, card);
        // First child paints under the content.
        if !self.disabled
            && let Some(wash) = ink::wash(
                &self.ink,
                self.key,
                state_layer(c.on_surface, theme.state.pressed),
                theme.shape.corner_full,
                Some([theme.shape.corner_medium; 4]),
            )
        {
            card = card.child(wash);
        }
        // Caller content in a keyed wrapper (only with ink): the wash layer
        // joins child 0, so unkeyed caller children would otherwise renumber
        // into new identities and replay their enter transitions.
        match (&self.ink, self.key) {
            (Some(_), Some(key)) => {
                card.child(col().key(format!("{key}:content")).children(self.children))
            }
            _ => card.children(self.children),
        }
    }
}
