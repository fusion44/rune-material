//! Desktop typography plus a session-local keyboard zoom override.

use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

use rune::{Key, KeyEvent, Tx, Waker};

use super::TypeScale;
use crate::{Theme, dms};

/// Dux's font scale multiplied by a local 50–200% zoom in 10% steps.
/// Defaults are deterministic; [`Self::from_desktop`] opts into desktop IO.
pub struct FontScale {
    system: f32,
    zoom_steps: i8,
    updates: Option<Receiver<f32>>,
}

impl Default for FontScale {
    fn default() -> Self {
        Self {
            system: 1.0,
            zoom_steps: 0,
            updates: None,
        }
    }
}

impl FontScale {
    pub fn from_desktop() -> Self {
        let system = std::fs::read(dms::typography_path())
            .ok()
            .and_then(|bytes| dms::font_scale_from_json(&bytes))
            .unwrap_or(1.0);
        Self {
            system,
            ..Self::default()
        }
    }

    /// Start off-thread watching after the first frame. No filesystem IO is
    /// performed by view, poll, or key handlers. Atomic file replacements work.
    pub fn watch(&mut self, waker: Waker) {
        let (sender, receiver) = mpsc::channel();
        self.updates = Some(receiver);
        let path = dms::typography_path();
        std::thread::spawn(move || {
            let mut last = None;
            loop {
                let bytes = std::fs::read(&path).ok();
                if bytes != last {
                    last = bytes;
                    if let Some(scale) = last.as_deref().and_then(dms::font_scale_from_json) {
                        if sender.send(scale).is_err() {
                            break;
                        }
                        waker();
                    }
                }
                std::thread::sleep(Duration::from_millis(250));
            }
        });
    }

    pub fn factor(&self) -> f32 {
        self.system * (1.0 + f32::from(self.zoom_steps) * 0.1)
    }

    /// Rebuild from baseline metrics, never compound an already-scaled theme.
    pub fn apply(&self, theme: &mut Theme) -> bool {
        let scale = TypeScale::material().scaled(self.factor());
        if theme.type_scale == scale {
            return false;
        }
        theme.type_scale = scale;
        true
    }

    /// Drain desktop updates and reapply after a palette/appearance change.
    pub fn poll(&mut self, theme: &mut Theme, tx: &mut Tx) -> bool {
        while let Some(scale) = self.updates.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.system = scale;
        }
        let changed = self.apply(theme);
        if changed {
            tx.instant();
        }
        changed
    }

    /// Ctrl++ (or Ctrl+=), Ctrl+-, Ctrl+0. Other modifier combinations pass
    /// through. Reset removes only the local override, not Dux's font scale.
    pub fn on_key(&mut self, event: &KeyEvent, theme: &mut Theme, tx: &mut Tx) -> bool {
        // `cmd` aliases Ctrl on Linux/Windows; it is not an extra held key.
        // Reject the physical Alt/Super modifiers, not the primary alias.
        if !event.mods.ctrl || event.mods.alt || event.mods.logo {
            return false;
        }
        let key = match &event.key {
            Key::Command(character) => *character,
            Key::Text(text) if text.len() == 1 => char::from(text.as_bytes()[0]),
            _ => return false,
        };
        match key {
            '+' | '=' => self.zoom_steps = (self.zoom_steps + 1).min(10),
            '-' => self.zoom_steps = (self.zoom_steps - 1).max(-5),
            '0' => self.zoom_steps = 0,
            _ => return false,
        }
        self.apply(theme);
        tx.instant();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_control_shortcuts_accept_primary_modifier_alias() {
        let mut fonts = FontScale::default();
        let mut theme = Theme::light();
        for (character, expected) in [('+', 1.1), ('=', 1.2), ('-', 1.1), ('0', 1.0)] {
            let event = KeyEvent {
                key: Key::Command(character),
                mods: rune::Mods {
                    ctrl: true,
                    cmd: true,
                    shift: character == '+',
                    ..Default::default()
                },
            };
            assert!(fonts.on_key(&event, &mut theme, &mut Tx::default()));
            assert!((fonts.factor() - expected).abs() < 1e-6);
        }
    }

    #[test]
    fn extra_physical_modifiers_do_not_zoom() {
        let mut fonts = FontScale::default();
        let mut theme = Theme::light();
        for mods in [
            rune::Mods {
                cmd: true,
                logo: true,
                ..Default::default()
            },
            rune::Mods {
                ctrl: true,
                cmd: true,
                alt: true,
                ..Default::default()
            },
            rune::Mods {
                ctrl: true,
                cmd: true,
                logo: true,
                ..Default::default()
            },
        ] {
            assert!(!fonts.on_key(
                &KeyEvent {
                    key: Key::Command('+'),
                    mods
                },
                &mut theme,
                &mut Tx::default(),
            ));
        }
        assert_eq!(fonts.factor(), 1.0);
    }

    fn key(fonts: &mut FontScale, theme: &mut Theme, text: &str) -> bool {
        fonts.on_key(
            &KeyEvent {
                key: Key::Text(text.into()),
                mods: rune::Mods {
                    ctrl: true,
                    ..Default::default()
                },
            },
            theme,
            &mut Tx::default(),
        )
    }

    #[test]
    fn desktop_and_zoom_multiply_without_compounding() {
        let mut fonts = FontScale {
            system: 1.2,
            ..Default::default()
        };
        let mut theme = Theme::light();
        fonts.apply(&mut theme);
        assert_eq!(theme.type_scale, TypeScale::material().scaled(1.2));
        assert!(!fonts.apply(&mut theme));
        assert!(key(&mut fonts, &mut theme, "+"));
        let expected = TypeScale::material().scaled(1.2 * 1.1);
        assert_eq!(theme.type_scale, expected);
        // Desktop updates preserve the local override.
        let (sender, receiver) = mpsc::channel();
        fonts.updates = Some(receiver);
        sender.send(1.5).unwrap();
        assert!(fonts.poll(&mut theme, &mut Tx::default()));
        assert_eq!(theme.type_scale, TypeScale::material().scaled(1.5 * 1.1));
        assert!(key(&mut fonts, &mut theme, "0"));
        assert_eq!(theme.type_scale, TypeScale::material().scaled(1.5));
    }

    #[test]
    fn zoom_is_bounded_and_plain_keys_pass_through() {
        let mut fonts = FontScale::default();
        let mut theme = Theme::light();
        for _ in 0..100 {
            key(&mut fonts, &mut theme, "=");
        }
        assert_eq!(fonts.factor(), 2.0);
        for _ in 0..100 {
            key(&mut fonts, &mut theme, "-");
        }
        assert_eq!(fonts.factor(), 0.5);
        assert!(!fonts.on_key(
            &KeyEvent {
                key: Key::Text("+".into()),
                mods: Default::default()
            },
            &mut theme,
            &mut Tx::default(),
        ));
        assert!(!key(&mut fonts, &mut theme, "1"));
        assert_eq!(theme.type_scale.body_medium.weight, 400.0);
        assert_eq!(theme.type_scale.body_medium.line_height, 10.0);
    }
}
