use rune::{Cx, El, Env, Runtime, el};
use rune_material::{ADD, IconButton, IconButtonVariant, Ink, Theme};

struct State {
    theme: Theme,
    ink: Ink,
    variant: IconButtonVariant,
    activated: bool,
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    el().w_full().h_full().center().child(
        IconButton::new(&ADD, "Add")
            .key("add")
            .variant(s.variant)
            .ink(&s.ink)
            .build(&s.theme)
            .on_press(|s: &mut State| s.activated = true),
    )
}

#[test]
fn edge_pressed_icon_ripples_remain_inside_the_button_and_release_normally() {
    for variant in [
        IconButtonVariant::Filled,
        IconButtonVariant::Tonal,
        IconButtonVariant::Outlined,
    ] {
        for offset in [(1.0, 20.0), (20.0, 20.0), (30.0, 30.0)] {
            let mut rt = Runtime::new(
                State {
                    theme: Theme::light(),
                    ink: Ink::new(),
                    variant,
                    activated: false,
                },
                view,
                (240.0, 160.0),
                Env::new(true, 1.0),
            );
            rt.poll = Some(|s, now, _| s.ink.poll(now));
            rt.frame(0.0);
            let ([x, y, w, h], _) = rt.presented("add").unwrap();
            let (px, py) = (x + offset.0, y + offset.1);
            rt.pointer_down(px, py);
            for _ in 0..12 {
                rt.frame(0.025);
                let elements = rt.elements();
                let ripple = elements
                    .iter()
                    .find(|e| e.key.as_deref() == Some("ink-add-0"))
                    .unwrap();
                let [rx, ry, rw, rh] = ripple.visible;
                assert!(
                    rx >= x - 0.05 && ry >= y - 0.05,
                    "ink escapes leading edges"
                );
                assert!(
                    rx + rw <= x + w + 0.05 && ry + rh <= y + h + 0.05,
                    "ink escapes trailing edges"
                );
            }
            rt.pointer_up(px, py);
            rt.frame(0.5);
            assert!(rt.state.activated);
            rt.frame(0.5);
            assert!(rt.state.ink.is_empty());
        }
    }
}
