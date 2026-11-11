//! Sliders from md.comp.slider tokens. The builder fixes the width and draws
//! the track and handle; the app attaches `.on_drag(..)` to map pointer
//! positions back to a value.

use rune::{El, el};

use crate::Theme;
use crate::components::{elevation, state_layer};

/// Builder for a Material 3 slider. `value` is a fraction from 0 to 1.
#[derive(Clone, Debug)]
pub struct Slider {
    value: f32,
    width: f32,
    label: Option<String>,
    disabled: bool,
    key: Option<&'static str>,
}

impl Slider {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            width: 240.0,
            label: None,
            disabled: false,
            key: None,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn key(mut self, key: &'static str) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self, theme: &Theme) -> El {
        let c = &theme.color;
        let travel = (self.width - 20.0).max(0.0);
        let handle_x = travel * self.value;
        let center = handle_x + 10.0;

        let (active, inactive, handle_color) = if self.disabled {
            (
                c.on_surface.alpha(0.38),
                c.on_surface.alpha(0.12),
                c.on_surface.alpha(0.38),
            )
        } else {
            (c.primary, c.surface_container_highest, c.primary)
        };

        let mut slider = el()
            .w(self.width)
            .h(40.0)
            .role("slider")
            .focusable(!self.disabled)
            .disabled(self.disabled)
            .range_value(self.value as f64, 0.0, 1.0)
            .range_step(0.01);
        if let Some(label) = self.label {
            slider = slider.label(label);
        }
        if let Some(key) = self.key {
            slider = slider.key(key);
        }

        slider = slider.child(
            el().key("inactive-track")
                .a11y_hidden()
                .pin(Some(10.0), Some(18.0), Some(10.0), None)
                .h(4.0)
                .radius(theme.shape.corner_full)
                .fill(inactive),
        );
        if self.value > 0.0 {
            slider = slider.child(
                el().key("active-track")
                    .a11y_hidden()
                    .pin(Some(10.0), Some(18.0), None, None)
                    .w(travel * self.value)
                    .h(4.0)
                    .radius(theme.shape.corner_full)
                    .fill(active),
            );
        }

        if !self.disabled {
            slider = slider.child(
                el().key("state-layer")
                    .a11y_hidden()
                    .absolute(center - 20.0, 0.0)
                    .size(40.0, 40.0)
                    .radius(theme.shape.corner_full)
                    .hover(state_layer(c.primary, theme.state.hover))
                    .press(state_layer(c.primary, theme.state.pressed)),
            );
        }

        slider = slider.child(elevation(
            el().key("handle")
                .a11y_hidden()
                .absolute(handle_x, 10.0)
                .size(20.0, 20.0)
                .radius(theme.shape.corner_full)
                .fill(handle_color),
            theme,
            if self.disabled { 0 } else { 1 },
        ));

        if !self.disabled {
            slider = slider.focus_ring(3.0, 2.0, c.secondary);
        } else {
            slider = slider
                .hover(rune::Color::TRANSPARENT)
                .press(rune::Color::TRANSPARENT)
                .no_focus_ring();
        }
        slider
    }
}
