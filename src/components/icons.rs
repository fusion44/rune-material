//! Official Material Symbols Rounded (Google, Apache-2.0).
//!
//! Weight 400, optical size 24, filled folders. Source revision, license and
//! regeneration instructions live in `assets/material-symbols/README.md`.
//! Apps can also pass their own `&'static Icon` to icon-bearing components.

use rune::Icon;

macro_rules! symbol {
    ($constant:ident, $name:literal) => {
        pub static $constant: Icon = Icon {
            name: concat!("material-rounded-", $name),
            view: 24.0,
            body: include_str!(concat!("../../assets/material-symbols/", $name, ".svg")),
        };
    };
}

symbol!(ADD, "add");
symbol!(ARCHIVE, "archive");
symbol!(ARROW_DROP_DOWN, "arrow_drop_down");
symbol!(CHECK, "check");
symbol!(CLOSE, "close");
symbol!(CODE, "code");
symbol!(DESCRIPTION, "description");
symbol!(DRAFT, "draft");
symbol!(FOLDER, "folder");
symbol!(FOLDER_OPEN, "folder_open");
symbol!(IMAGE, "image");
symbol!(LOCK, "lock");
symbol!(PLAY_CIRCLE, "play_circle");
symbol!(REMOVE, "remove");
symbol!(SEARCH, "search");

#[cfg(test)]
#[path = "icons_tests.rs"]
mod tests;
