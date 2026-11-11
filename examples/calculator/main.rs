//! Material 3 calculator built from `rune-material`, sharing the DMS watcher
//! pattern from the `dms-theme` example so it follows the host palette
//! near-realtime. Fixed 360×540 layout per the comparison spec.
//!
//! Layout: display card on top, then a 4-column keypad (C Del % ÷ / 7 8 9 × /
//! 4 5 6 − / 1 2 3 + / ± 0 . =). Digits are tonal, operators outlined, clear
//! text, equals filled. Keyboard: digits, `+ - * / % .`, Enter/=, Escape=C,
//! Backspace=Del. ("Del" is text because the Inter slot has no ⌫ glyph.)

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};

use rune::{AppSpec, Cx, El, Key, KeyEvent, col, row, text};
use rune_material::Ink;
use rune_material::{
    Button, ButtonVariant, Card, CardVariant, Theme,
    dms::{self, theme_from_json},
};

struct State {
    theme: Theme,
    dark: bool,
    path: PathBuf,
    rx: Option<Receiver<(Theme, bool)>>,
    ink: Ink,
    entry: String,
    acc: Option<f64>,
    op: Option<char>,
    fresh: bool,
}

fn init() -> State {
    let path = dms::palette_path();
    let (theme, dark) = std::fs::read(&path)
        .ok()
        .and_then(|b| theme_from_json(&b, None))
        .unwrap_or((Theme::dark(), true));
    State {
        theme,
        dark,
        path,
        rx: None,
        ink: Ink::new(),
        entry: "0".into(),
        acc: None,
        op: None,
        fresh: true,
    }
}

fn setup(s: &mut State, waker: rune::Waker) {
    let (tx, rx) = mpsc::channel::<(Theme, bool)>();
    s.rx = Some(rx);
    let path = s.path.clone();
    let initial = std::fs::read(&path).ok();
    std::thread::spawn(move || {
        let mut last: Option<Vec<u8>> = initial;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(200));
            let bytes = std::fs::read(&path).ok();
            if bytes == last {
                continue;
            }
            last = bytes.clone();
            let Some(bytes) = bytes else { continue };
            let Some((theme, dark)) = theme_from_json(&bytes, None) else {
                continue;
            };
            if tx.send((theme, dark)).is_err() {
                break;
            }
            waker();
        }
    });
}

fn poll(s: &mut State, _now: f64, _tx: &mut rune::Tx) -> bool {
    let mut changed = s.ink.poll(_now);
    while let Some((theme, dark)) =
        s.rx.as_ref()
            .map(|rx| rx.try_recv())
            .transpose()
            .ok()
            .flatten()
    {
        if theme.color == s.theme.color && dark == s.dark {
            continue;
        }
        s.theme = theme;
        s.dark = dark;
        changed = true;
    }
    changed
}

impl State {
    fn value(&self) -> f64 {
        self.entry.parse::<f64>().unwrap_or(0.0)
    }

    fn render(&mut self, v: f64) {
        self.entry = format_number(v);
        self.fresh = true;
    }

    fn input_digit(&mut self, d: char) {
        if self.fresh {
            self.entry = if d == '0' { "0".into() } else { d.to_string() };
            self.fresh = false;
        } else if self.entry.replace(['-', '.'], "").len() < 12 {
            if self.entry == "0" {
                self.entry = d.to_string();
            } else if self.entry == "-0" {
                self.entry = format!("-{d}");
            } else {
                self.entry.push(d);
            }
        }
    }

    fn input_dot(&mut self) {
        if self.fresh {
            self.entry = "0.".into();
            self.fresh = false;
        } else if !self.entry.contains('.') {
            self.entry.push('.');
        }
    }

    fn apply(op: char, a: f64, b: f64) -> f64 {
        match op {
            '+' => a + b,
            '−' => a - b,
            '×' => a * b,
            '÷' => {
                if b == 0.0 {
                    f64::NAN
                } else {
                    a / b
                }
            }
            _ => b,
        }
    }

    fn set_op(&mut self, op: char) {
        let v = self.value();
        if let (Some(acc), Some(pending)) = (self.acc, self.op) {
            if !self.fresh {
                let r = Self::apply(pending, acc, v);
                self.acc = Some(r);
                self.render(r);
            }
        } else {
            self.acc = Some(v);
        }
        self.op = Some(op);
        self.fresh = true;
    }

    fn equals(&mut self) {
        if let (Some(acc), Some(op)) = (self.acc, self.op) {
            let r = Self::apply(op, acc, self.value());
            self.acc = None;
            self.op = None;
            self.render(r);
        }
    }

    fn clear(&mut self) {
        self.entry = "0".into();
        self.acc = None;
        self.op = None;
        self.fresh = true;
    }

    fn backspace(&mut self) {
        if self.fresh {
            return;
        }
        self.entry.pop();
        while self.entry.ends_with(['.', 'e']) {
            self.entry.pop();
        }
        if self.entry.is_empty() || self.entry == "-" {
            self.entry = "0".into();
            self.fresh = true;
        }
    }
}

fn format_number(v: f64) -> String {
    if v.is_nan() {
        return "Error".into();
    }
    if v.abs() >= 1e12 || (v != 0.0 && v.abs() < 1e-9) {
        return format!("{v:.5e}");
    }
    let mut s = format!("{v:.8}");
    while s.contains('.') && (s.ends_with('0') || s.ends_with('.')) {
        if s.ends_with('0') {
            s.pop();
        } else {
            s.pop();
            break;
        }
    }
    if s.is_empty() || s == "-0" {
        "0".into()
    } else {
        s
    }
}

fn key_button(
    label: &str,
    key: &'static str,
    variant: ButtonVariant,
    t: &Theme,
    ink: &Ink,
    press: impl Fn(&mut State) + 'static,
) -> El {
    Button::new(label)
        .variant(variant)
        .key(key)
        .ink(ink)
        .build(t)
        .grow(1.0)
        .on_press(press)
}

fn key_row(_t: &Theme, cells: Vec<El>) -> El {
    row().w_full().gap(8.0).children(cells)
}

fn keypad(s: &State) -> El {
    let t = &s.theme;
    let tonal = ButtonVariant::Tonal;
    let outlined = ButtonVariant::Outlined;
    col()
        .w_full()
        .gap(8.0)
        .child(key_row(
            t,
            vec![
                key_button(
                    "C",
                    "key-c",
                    ButtonVariant::Text,
                    t,
                    &s.ink,
                    |s: &mut State| {
                        s.clear();
                    },
                ),
                key_button("Del", "key-back", outlined, t, &s.ink, |s: &mut State| {
                    s.backspace();
                }),
                key_button("%", "key-pct", outlined, t, &s.ink, |s: &mut State| {
                    let v = s.value() / 100.0;
                    s.render(v);
                }),
                key_button("÷", "key-div", outlined, t, &s.ink, |s: &mut State| {
                    s.set_op('÷');
                }),
            ],
        ))
        .child(key_row(
            t,
            vec![
                key_button("7", "key-7", tonal, t, &s.ink, |s: &mut State| {
                    s.input_digit('7')
                }),
                key_button("8", "key-8", tonal, t, &s.ink, |s: &mut State| {
                    s.input_digit('8')
                }),
                key_button("9", "key-9", tonal, t, &s.ink, |s: &mut State| {
                    s.input_digit('9')
                }),
                key_button("×", "key-mul", outlined, t, &s.ink, |s: &mut State| {
                    s.set_op('×');
                }),
            ],
        ))
        .child(key_row(
            t,
            vec![
                key_button("4", "key-4", tonal, t, &s.ink, |s: &mut State| {
                    s.input_digit('4')
                }),
                key_button("5", "key-5", tonal, t, &s.ink, |s: &mut State| {
                    s.input_digit('5')
                }),
                key_button("6", "key-6", tonal, t, &s.ink, |s: &mut State| {
                    s.input_digit('6')
                }),
                key_button("−", "key-sub", outlined, t, &s.ink, |s: &mut State| {
                    s.set_op('−');
                }),
            ],
        ))
        .child(key_row(
            t,
            vec![
                key_button("1", "key-1", tonal, t, &s.ink, |s: &mut State| {
                    s.input_digit('1')
                }),
                key_button("2", "key-2", tonal, t, &s.ink, |s: &mut State| {
                    s.input_digit('2')
                }),
                key_button("3", "key-3", tonal, t, &s.ink, |s: &mut State| {
                    s.input_digit('3')
                }),
                key_button("+", "key-add", outlined, t, &s.ink, |s: &mut State| {
                    s.set_op('+');
                }),
            ],
        ))
        .child(key_row(
            t,
            vec![
                key_button("±", "key-neg", outlined, t, &s.ink, |s: &mut State| {
                    let v = -s.value();
                    s.render(v);
                    s.fresh = false;
                }),
                key_button("0", "key-0", tonal, t, &s.ink, |s: &mut State| {
                    s.input_digit('0')
                }),
                key_button(".", "key-dot", tonal, t, &s.ink, |s: &mut State| {
                    s.input_dot()
                }),
                key_button(
                    "=",
                    "key-eq",
                    ButtonVariant::Filled,
                    t,
                    &s.ink,
                    |s: &mut State| {
                        s.equals();
                    },
                ),
            ],
        ))
}

fn on_key(s: &mut State, event: &KeyEvent, _tx: &mut rune::Tx) -> bool {
    match &event.key {
        Key::Text(t) => {
            let c = t.chars().next().unwrap_or(' ');
            match c {
                '0'..='9' => {
                    s.input_digit(c);
                    true
                }
                '.' | ',' => {
                    s.input_dot();
                    true
                }
                '+' => {
                    s.set_op('+');
                    true
                }
                '-' => {
                    s.set_op('−');
                    true
                }
                '*' => {
                    s.set_op('×');
                    true
                }
                '/' => {
                    s.set_op('÷');
                    true
                }
                '%' => {
                    let v = s.value() / 100.0;
                    s.render(v);
                    true
                }
                '=' => {
                    s.equals();
                    true
                }
                _ => false,
            }
        }
        Key::Enter => {
            s.equals();
            true
        }
        Key::Escape => {
            s.clear();
            true
        }
        Key::Backspace => {
            s.backspace();
            true
        }
        _ => false,
    }
}

fn scene_light(s: &mut State) {
    (s.theme, s.dark) = (Theme::light(), false);
}

fn scene_dark(s: &mut State) {
    (s.theme, s.dark) = (Theme::dark(), true);
}

fn scene_sum(s: &mut State) {
    scene_light(s);
    s.entry = "42".into();
    s.acc = Some(12.0);
    s.op = Some('+');
    s.fresh = true;
}

type Scene = (&'static str, fn(&mut State));
const SCENES: &[Scene] = &[
    ("light", scene_light),
    ("dark", scene_dark),
    ("sum", scene_sum),
];

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = &s.theme;
    let c = &t.color;
    let pending = match (s.acc, s.op) {
        (Some(acc), Some(op)) => format!("{} {op}", format_number(acc)),
        _ => String::new(),
    };
    col()
        .w_full()
        .h_full()
        .padding(16.0)
        .gap(12.0)
        .fill(c.surface)
        .child(
            Card::new()
                .variant(CardVariant::Filled)
                .key("calc-display")
                .padding(16.0)
                .child(
                    col()
                        .w_full()
                        .gap(4.0)
                        .child(
                            row().w_full().justify_end().child(
                                text(pending)
                                    .font(t.type_scale.body_medium)
                                    .color(c.on_surface_variant),
                            ),
                        )
                        .child(
                            row().w_full().justify_end().child(
                                text(s.entry.clone())
                                    .key("calc-entry")
                                    .font(t.type_scale.display_small)
                                    .color(c.on_surface),
                            ),
                        ),
                )
                .build(t),
        )
        .child(keypad(s))
}

fn main() {
    rune::run(AppSpec::<State> {
        setup: Some(setup),
        poll: Some(poll),
        on_key: Some(on_key),
        scenes: SCENES,
        ..AppSpec::new("material calculator", (360.0, 470.0), init, view)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> State {
        State {
            theme: Theme::light(),
            dark: false,
            path: PathBuf::new(),
            rx: None,
            ink: Ink::new(),
            entry: "0".into(),
            acc: None,
            op: None,
            fresh: true,
        }
    }

    #[test]
    fn applies_four_operations() {
        assert_eq!(State::apply('+', 12.0, 30.0), 42.0);
        assert_eq!(State::apply('−', 12.0, 30.0), -18.0);
        assert_eq!(State::apply('×', 6.0, 7.0), 42.0);
        assert_eq!(State::apply('÷', 84.0, 2.0), 42.0);
        assert!(State::apply('÷', 1.0, 0.0).is_nan());
    }

    #[test]
    fn formats_results() {
        assert_eq!(format_number(42.0), "42");
        assert_eq!(format_number(-0.5), "-0.5");
        assert_eq!(format_number(f64::NAN), "Error");
        assert_eq!(format_number(1e13), "1.00000e13");
    }

    #[test]
    fn chains_operations_like_a_pocket_calculator() {
        let mut s = state();
        s.input_digit('1');
        s.input_digit('2');
        s.set_op('+');
        s.input_digit('3');
        s.input_digit('0');
        s.equals();
        assert_eq!(s.entry, "42");
        assert_eq!(s.op, None);
    }

    #[test]
    fn clear_and_backspace() {
        let mut s = state();
        s.input_digit('4');
        s.input_digit('2');
        s.backspace();
        assert_eq!(s.entry, "4");
        s.clear();
        assert_eq!(s.entry, "0");
        assert_eq!(s.acc, None);
    }
}
