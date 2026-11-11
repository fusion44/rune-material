//! DMS-reactive demo: watches `$XDG_CACHE_HOME/dms/dms-colors.json` on a
//! background thread and rebuilds the Material theme near-realtime.
//!
//! The watcher polls the file's bytes every 200 ms (the atomic `.tmp` +
//! rename write means bytes are never torn), parses with
//! [`rune_material::dms`], and wakes Rune via `setup`'s [`rune::Waker`].
//! [`poll`] drains the channel and returns whether the theme changed.
//! `DMS_COLORS_FILE` overrides the palette path; scenes render baselines for
//! `--snap`.

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::time::{SystemTime, UNIX_EPOCH};

use rune::{AppSpec, Cx, El, col, row, text};
use rune_material::Ink;
use rune_material::{
    Button, ButtonVariant, Card, Switch, Theme,
    dms::{self, theme_from_json},
};

struct State {
    theme: Theme,
    fonts: rune_material::FontScale,
    dark: bool,
    path: PathBuf,
    generation: u64,
    status: String,
    rx: Option<Receiver<(Theme, bool, u64, String)>>,
    ink: Ink,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn init() -> State {
    let path = dms::palette_path();
    let (mut theme, dark, status) = match std::fs::read(&path) {
        Ok(bytes) => match theme_from_json(&bytes, None) {
            Some((theme, dark)) => (theme, dark, format!("loaded {}", path.display())),
            None => (
                Theme::dark(),
                true,
                format!("unparseable {}", path.display()),
            ),
        },
        Err(_) => (
            Theme::dark(),
            true,
            format!("missing {}, using baseline dark", path.display()),
        ),
    };
    let fonts = rune_material::FontScale::from_desktop();
    fonts.apply(&mut theme);
    State {
        theme,
        fonts,
        dark,
        path,
        generation: 0,
        status,
        rx: None,
        ink: Ink::new(),
    }
}

fn setup(s: &mut State, waker: rune::Waker) {
    s.fonts.watch(waker.clone());
    let (tx, rx) = mpsc::channel::<(Theme, bool, u64, String)>();
    s.rx = Some(rx);
    let path = s.path.clone();
    let initial = std::fs::read(&path).ok();
    std::thread::spawn(move || {
        let mut last: Option<Vec<u8>> = initial;
        let mut generation = 0u64;
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
            generation += 1;
            let status = format!("refresh #{} at {}s", generation, now_secs());
            if tx.send((theme, dark, generation, status)).is_err() {
                break;
            }
            waker();
        }
    });
}

fn poll(s: &mut State, _now: f64, _tx: &mut rune::Tx) -> bool {
    let mut changed = s.ink.poll(_now);
    while let Some((theme, dark, generation, status)) =
        s.rx.as_ref()
            .map(|rx| rx.try_recv())
            .transpose()
            .ok()
            .flatten()
    {
        // Skip palettes identical to the live one (debounce by content).
        let mut theme = theme;
        s.fonts.apply(&mut theme);
        if theme == s.theme && dark == s.dark {
            continue;
        }
        s.theme = theme;
        s.dark = dark;
        s.generation = generation;
        s.status = status;
        changed = true;
    }
    s.fonts.poll(&mut s.theme, _tx) || changed
}

fn scene_light(s: &mut State) {
    s.fonts = Default::default();
    s.theme = Theme::light();
    s.dark = false;
    s.status = "baseline light (snapshot)".into();
}

fn scene_dark(s: &mut State) {
    s.fonts = Default::default();
    s.theme = Theme::dark();
    s.dark = true;
    s.status = "baseline dark (snapshot)".into();
}

type Scene = (&'static str, fn(&mut State));
const SCENES: &[Scene] = &[("light", scene_light), ("dark", scene_dark)];

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = &s.theme;
    let c = &t.color;
    col()
        .w_full()
        .h_full()
        .padding(24.0)
        .gap(16.0)
        .fill(c.surface)
        .child(
            text("DMS theme demo")
                .font(t.type_scale.headline_small)
                .color(c.on_surface),
        )
        .child(
            text(format!(
                "{} · generation {} · {}",
                if s.dark { "dark" } else { "light" },
                s.generation,
                s.status
            ))
            .font(t.type_scale.body_medium)
            .color(c.on_surface_variant),
        )
        .child(
            text(s.path.display().to_string())
                .font(t.type_scale.body_small)
                .color(c.on_surface_variant),
        )
        .child(
            Card::new()
                .key("dms-card")
                .padding(16.0)
                .child(
                    col()
                        .gap(12.0)
                        .child(
                            row()
                                .gap(8.0)
                                .center_y()
                                .child(
                                    Button::new("Filled")
                                        .key("dms-filled")
                                        .ink(&s.ink)
                                        .build(t)
                                        .on_press(|_: &mut State| {}),
                                )
                                .child(
                                    Button::new("Tonal")
                                        .variant(ButtonVariant::Tonal)
                                        .key("dms-tonal")
                                        .ink(&s.ink)
                                        .build(t)
                                        .on_press(|_: &mut State| {}),
                                )
                                .child(
                                    Button::new("Outlined")
                                        .variant(ButtonVariant::Outlined)
                                        .key("dms-outlined")
                                        .ink(&s.ink)
                                        .build(t)
                                        .on_press(|_: &mut State| {}),
                                ),
                        )
                        .child(
                            Switch::new(s.dark, if s.dark { "Dark" } else { "Light" })
                                .key("dms-switch")
                                .build(t)
                                .on_press(|_: &mut State| {}),
                        ),
                )
                .build(t),
        )
        .child(
            text("Edit the palette (or change DMS theme) and this window follows within ~200 ms.")
                .font(t.type_scale.body_small)
                .color(c.on_surface_variant),
        )
}

fn main() {
    rune::run(AppSpec::<State> {
        setup: Some(setup),
        on_key: Some(|s, event, tx| s.fonts.on_key(event, &mut s.theme, tx)),
        poll: Some(poll),
        scenes: SCENES,
        ..AppSpec::new("dms theme demo", (640.0, 480.0), init, view)
    });
}
