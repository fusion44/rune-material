//! Material ink ripples: the expanding wash a press spreads across a control.
//!
//! A pressable opts in with `.ink(&ink)`, sharing the app's [`Ink`] store:
//!
//! ```ignore
//! struct State {
//!     ink: Ink,
//!     // ...
//! }
//!
//! Button::new("Save")
//!     .key("save")
//!     .ink(&s.ink)
//!     .build(&theme)
//!     .on_press(|s: &mut State| s.save())
//! ```
//!
//! How it works, following Rune's own click-ripple overlay
//! (`rune::input::keycaps`): the builder attaches an `on_drag` origin capture
//! (pointer-down position in element points, release on pointer-up) and paints
//! one keyed circle per live press. Enter runs `scale(0.5)` (springs out from
//! the touch point) and expiry unmounts through `opacity()` (fades). The
//! motion system drives both; no per-frame app code is needed. Expiry is
//! time-based, so the app must drain the store from its poll:
//!
//! ```ignore
//! fn poll(s: &mut State, now: f64, _: &mut rune::Tx) -> bool {
//!     s.ink.poll(now)
//! }
//! ```
//!
//! Builder content carries derived `{key}:<slot>` keys, so the wash layer
//! joining child 0 never renumbers siblings into fresh identities (which
//! would replay their enter transitions mid-press).
//!
//! A ripple lives while its press is held, then fades over [`FADE_SECS`] once
//! released; a press whose release is lost (pointer capture stolen) is
//! reaped after [`MAX_HOLD_SECS`].
//!
//! Notes and limits:
//!
//! - Ripples match by element key: set `.key(..)` or no ripple appears.
//! - `.ink()` occupies the element's `on_drag` slot for origin capture. An app
//!   `on_drag` attached after `build()` replaces it (silently disabling the
//!   ripple), so `Slider` — whose API *is* app-attached `on_drag` — has no
//!   `.ink()`.
//! - Keyboard activation has no pointer origin and shows no ripple; the focus
//!   ring and state layer still respond.
//! - Without a wired `poll`, ripples linger (capped at [`MAX_RIPPLES`]).
//! - Built-in controls clip ripples to their host shape, including circular
//!   icon buttons. Focus rings are independent and can extend outside it.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use rune::{Color, Drag, El, Phase, Transition, Tx, el};

/// Fade-out after release, in seconds. Covers the exit fade with margin.
const FADE_SECS: f64 = 0.2;
/// Backstop lifetime for a press whose release is never seen (capture lost),
/// in seconds. Held presses stay mounted until released.
const MAX_HOLD_SECS: f64 = 2.0;
/// Cap per store, so an unwired `poll` cannot grow the tree forever.
const MAX_RIPPLES: usize = 8;
/// Enter transition starts here and springs to full size. Rune's kit rules
/// clamp transition scales to [0.9, 1.1] — `allow_small_scale` (see below)
/// extends the floor to 0.5, the smallest a spread may start from.
/// `Transition::scale` also fades in, which would dilute the spread, so the
/// insertion runs at full ink opacity and only grows.
const ENTER_SCALE: f32 = 0.5;
/// Smallest ripple radius, so edge presses still read.
const MIN_RADIUS: f32 = 12.0;

#[derive(Clone, Debug)]
struct Ripple {
    id: u64,
    key: String,
    x: f32,
    y: f32,
    r: f32,
    t0: f64,
    released: Option<f64>,
}

#[derive(Debug, Default)]
struct Inner {
    ripples: Vec<Ripple>,
    next: u64,
}

/// One mounted ink circle, in element-layout points. Returned by
/// [`Ink::ripples`] for custom painters; builders use [`wash`] instead.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InkRipple {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

/// App-owned ink-ripple store. `Clone` shares the same presses; interior
/// mutability fits `view(&S)` (shared state) and Rune's `Rc`-based handlers.
#[derive(Clone, Debug, Default)]
pub struct Ink {
    inner: Rc<RefCell<Inner>>,
}

impl Ink {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a live press, scheduling a backstop wake through `tx`. Called
    /// from the builder-attached origin capture; apps reaching for this
    /// directly should prefer fixture-only [`Ink::press_at`].
    pub fn press(&self, key: &str, x: f32, y: f32, w: f32, h: f32, tx: &mut Tx) {
        let now = tx.clock();
        self.press_at(key, x, y, w, h, now);
        tx.wake_after(Duration::from_millis((MAX_HOLD_SECS * 1000.0) as u64 + 50));
    }

    /// Record a release for `key`'s latest held press, scheduling its fade.
    /// Cancelled gestures release the same way (their ripple fades rather
    /// than vanishing). No live press: no-op.
    pub fn release(&self, key: &str, now: f64, tx: &mut Tx) {
        if self.release_at(key, now) {
            tx.wake_after(Duration::from_millis((FADE_SECS * 1000.0) as u64 + 50));
        }
    }

    /// Record a release at wall-clock `now` without scheduling. For tests;
    /// live releases go through [`Ink::release`]. Returns whether a held
    /// press was actually released.
    pub fn release_at(&self, key: &str, now: f64) -> bool {
        let mut inner = self.inner.borrow_mut();
        let Some(r) = inner
            .ripples
            .iter_mut()
            .rev()
            .find(|r| r.key == key && r.released.is_none())
        else {
            return false;
        };
        r.released = Some(now);
        true
    }

    /// Record a press at wall-clock `now`, without scheduling expiry. For
    /// snapshot fixtures and tests; live presses go through [`Ink::press`].
    pub fn press_at(&self, key: &str, x: f32, y: f32, w: f32, h: f32, now: f64) {
        let corners = [(0.0, 0.0), (w, 0.0), (0.0, h), (w, h)];
        let r = corners
            .iter()
            .map(|(cx, cy)| (cx - x).hypot(cy - y))
            .fold(MIN_RADIUS, f32::max);
        let mut inner = self.inner.borrow_mut();
        let id = inner.next;
        inner.next += 1;
        inner.ripples.push(Ripple {
            id,
            key: key.into(),
            x,
            y,
            r,
            t0: now,
            released: None,
        });
        while inner.ripples.len() > MAX_RIPPLES {
            inner.ripples.remove(0);
        }
    }

    /// Drop expired ripples: released ones past their fade, and any press
    /// older than the backstop. Call from the app's `poll(now)` and return its
    /// value: `true` rebuilds so exits can fade.
    pub fn poll(&self, now: f64) -> bool {
        let mut inner = self.inner.borrow_mut();
        let before = inner.ripples.len();
        inner.ripples.retain(|r| {
            now - r.t0 < MAX_HOLD_SECS && r.released.is_none_or(|rel| now - rel < FADE_SECS)
        });
        inner.ripples.len() != before
    }

    /// Live ripples for one element key, oldest first.
    pub fn ripples(&self, key: &str) -> Vec<InkRipple> {
        self.inner
            .borrow()
            .ripples
            .iter()
            .filter(|r| r.key == key)
            .map(|r| InkRipple {
                id: r.id,
                x: r.x,
                y: r.y,
                r: r.r,
            })
            .collect()
    }

    /// Whether any ripple is mounted (tests).
    pub fn is_empty(&self) -> bool {
        self.inner.borrow().ripples.is_empty()
    }
}

/// Attach pointer-down origin capture to `el`. Needs the element key; without
/// one (or when disabled) the element is returned unchanged. The key is only
/// borrowed for matching — dynamic keys (like file paths) are fine.
pub(crate) fn capture(ink: &Option<Ink>, key: Option<&str>, disabled: bool, el: El) -> El {
    let (Some(ink), Some(key)) = (ink, key) else {
        return el;
    };
    if disabled {
        return el;
    }
    let ink = ink.clone();
    let key = key.to_owned();
    el.on_drag_any(move |d: &mut Drag, tx: &mut Tx| match d.phase {
        Phase::Down => ink.press(&key, d.x, d.y, d.w, d.h, tx),
        // Up also ends a cancelled gesture: its ripple fades rather than
        // vanishing, matching a released press.
        Phase::Up => ink.release(&key, tx.clock(), tx),
        Phase::Move => {}
    })
}

/// The wash layer for one element: an absolute fill of the host with one
/// keyed circle per live press. Returns `None` when there is nothing to
/// paint, keeping the tree clean. `round` is the circle radius token.
/// `clip` is the host's corner radii for a bounded wash, or `None` for an
/// unbounded custom wash that may spill past the host box.
/// Splice the layer as the FIRST child so content paints above the ink.
pub(crate) fn wash(
    ink: &Option<Ink>,
    key: Option<&str>,
    color: Color,
    round: f32,
    clip: Option<[f32; 4]>,
) -> Option<El> {
    let (Some(ink), Some(key)) = (ink, key) else {
        return None;
    };
    let ripples = ink.ripples(key);
    if ripples.is_empty() {
        return None;
    }
    let mut layer = el()
        .key(format!("inklayer-{key}"))
        .a11y_hidden()
        .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0))
        // Identity: the layer itself must not enter. It defaults to an opacity
        // fade, which the circles would ride instead of playing their own
        // scale entrance (they appear at full size, fading with the layer).
        .transition(Transition::none());
    if let Some(radii) = clip {
        layer = layer.clip_corners(radii);
    }
    for r in ripples {
        layer = layer.child(
            el().key(format!("ink-{key}-{}", r.id))
                .a11y_hidden()
                .absolute(r.x - r.r, r.y - r.r)
                .size(2.0 * r.r, 2.0 * r.r)
                .radius(round)
                .fill(color)
                .transition(enter()),
        );
    }
    Some(layer)
}

/// The ripple enter/exit: grow from half size at full ink opacity over the
/// settle spring (the exit fades in place). `Transition::scale` also fades
/// in, which would dilute the spread, so the insertion runs opaque.
fn enter() -> Transition {
    let mut t = Transition::asymmetric(Transition::scale(ENTER_SCALE), Transition::opacity());
    t.insertion.opacity = 1.0;
    t.allow_small_scale()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radius_reaches_the_farthest_corner() {
        let ink = Ink::new();
        ink.press_at("b", 60.0, 20.0, 120.0, 40.0, 0.0);
        let ripples = ink.ripples("b");
        assert_eq!(ripples.len(), 1);
        assert!((ripples[0].r - 63.25).abs() < 0.1, "got {}", ripples[0].r);
        assert!(ink.ripples("other").is_empty(), "ripples match by key");
    }

    #[test]
    fn poll_expires_released_ripples_after_the_fade() {
        let ink = Ink::new();
        ink.press_at("b", 10.0, 10.0, 40.0, 40.0, 100.0);
        assert!(ink.release_at("b", 100.1));
        assert!(!ink.poll(100.29));
        assert!(!ink.is_empty());
        assert!(ink.poll(100.31));
        assert!(ink.is_empty());
    }

    #[test]
    fn held_ripples_outlive_taps_but_not_the_backstop() {
        let ink = Ink::new();
        ink.press_at("b", 10.0, 10.0, 40.0, 40.0, 100.0);
        // Still held a full second later: no fade scheduled.
        assert!(!ink.poll(101.0));
        assert!(!ink.is_empty());
        // A lost release is reaped by the backstop.
        assert!(!ink.poll(101.99));
        assert!(ink.poll(102.01));
        assert!(ink.is_empty());
    }

    #[test]
    fn release_marks_only_the_latest_held_press() {
        let ink = Ink::new();
        ink.press_at("b", 1.0, 1.0, 8.0, 8.0, 100.0);
        ink.press_at("b", 2.0, 2.0, 8.0, 8.0, 100.1);
        assert!(ink.release_at("b", 100.2));
        assert!(!ink.poll(100.35));
        assert_eq!(ink.ripples("b").len(), 2, "released ripple still fading");
        assert!(ink.poll(100.45), "fade window ends the released ripple");
        assert_eq!(ink.ripples("b").len(), 1, "held press survives");
        assert!(!ink.release_at("other", 100.3), "unknown keys are a no-op");
    }

    #[test]
    fn the_store_is_capped() {
        let ink = Ink::new();
        for i in 0..(MAX_RIPPLES + 4) {
            ink.press_at("b", 1.0, 1.0, 8.0, 8.0, i as f64 * 10.0);
        }
        assert_eq!(ink.inner.borrow().ripples.len(), MAX_RIPPLES);
    }
}
