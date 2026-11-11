//! Material 3 components. Builders read only semantic `Theme` tokens and end in
//! `.build(&theme) -> El`.

pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod dialog;
pub mod divider;
pub mod fab;
pub mod icon_button;
pub mod icons;
pub mod list;
pub mod menu;
pub mod progress;
pub mod radio;
pub mod select;
pub mod slider;
pub mod snackbar;
pub mod switch;
pub mod tabs;
pub mod text_field;

pub use badge::Badge;
pub use card::{Card, CardVariant};
pub use dialog::{Dialog, Scrim};
pub use divider::Divider;
pub use list::{List, ListItem};
pub use menu::{Menu, MenuItem};
pub use progress::Progress;
pub use slider::Slider;
pub use snackbar::{Snackbar, SnackbarAction};
pub use tabs::{Tab, TabVariant, Tabs};

pub(crate) fn state_layer(color: rune::Color, opacity: f32) -> rune::Color {
    color.alpha(opacity)
}

pub(crate) fn elevation(el: rune::El, theme: &crate::Theme, level: u8) -> rune::El {
    let Some([key, ambient]) = theme.elevation.shadow(level, theme.color.shadow) else {
        return el;
    };
    el.box_shadow(key).box_shadow(ambient)
}
