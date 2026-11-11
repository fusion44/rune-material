use super::*;
use rune::{Align, Env, Key, KeyEvent, Mods, Runtime, Side};

fn runtime(size: (f32, f32)) -> Runtime<State> {
    let mut s = init();
    scene_light(&mut s);
    let mut rt = Runtime::new(s, view, size, Env::new(true, 1.0));
    rt.poll = Some(poll);
    rt.frame(0.0);
    rt
}

fn reveal(rt: &mut Runtime<State>, key: &'static str) {
    rt.update_if(|_, tx| {
        tx.scroll_into_view_in("gallery-scroll", key);
        tx.instant();
        true
    });
    rt.frame(0.0);
    rt.frame(1.0);
}

fn click(rt: &mut Runtime<State>, key: &str) {
    let (x, y) = rt.find(key).unwrap();
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(0.0);
    rt.frame(1.0);
}

fn close(a: [f32; 4], b: [f32; 4]) {
    assert!(
        a.iter().zip(b).all(|(a, b)| (a - b).abs() < 0.1),
        "{a:?} != {b:?}"
    );
}

#[test]
fn preopened_menu_scene_keeps_its_trigger_and_popup_on_screen() {
    let mut s = init();
    scene_menu(&mut s);
    let mut rt = Runtime::new(s, view, (900.0, 1000.0), Env::new(true, 1.0));
    rt.frame(0.0);
    rt.frame(0.5);
    rt.frame(0.5);
    for key in ["open-menu", "menu"] {
        let ([x, y, w, h], _) = rt.presented(key).unwrap();
        assert!(
            x >= 0.0 && y >= 0.0 && x + w <= 900.1 && y + h <= 1000.1,
            "{key} is visible"
        );
    }
}

#[test]
fn menu_opens_next_to_its_scrolled_trigger_and_tracks_window_size() {
    for size in [(420.0, 520.0), (900.0, 1000.0)] {
        let mut rt = runtime(size);
        reveal(&mut rt, "open-menu");
        click(&mut rt, "open-menu");
        assert!(rt.state.menu_open);
        for size in [size, (700.0, 600.0)] {
            rt.set_viewport(size, 1.0);
            rt.frame(0.0);
            rt.frame(1.0);
            let ([x, y, w, h], _) = rt.presented("open-menu").unwrap();
            let (menu, _) = rt.presented("menu").unwrap();
            let expected = rune::anchor_rect(
                [x, y, x + w, y + h],
                (menu[2], menu[3]),
                Side::Bottom,
                Align::Start,
                8.0,
                size,
            );
            close(
                menu,
                [
                    expected[0],
                    expected[1],
                    expected[2] - expected[0],
                    expected[3] - expected[1],
                ],
            );
            assert!(menu[0] >= 7.9 && menu[1] >= 7.9);
            assert!(menu[0] + menu[2] <= size.0 - 7.9);
            assert!(
                menu[1] + menu[3] <= size.1 - 7.9,
                "menu {menu:?}, trigger {:?}, viewport {size:?}",
                [x, y, w, h]
            );
        }
        rt.key(KeyEvent {
            key: Key::Escape,
            mods: Mods::default(),
        });
        rt.frame(0.0);
        assert!(!rt.state.menu_open, "modal dismissal still works");
    }
}

#[test]
fn dialog_backdrop_fills_window_without_reflowing_gallery() {
    for size in [(420.0, 520.0), (900.0, 1000.0), (1200.0, 700.0)] {
        let mut rt = runtime(size);
        reveal(&mut rt, "open-dialog");
        let before = rt.presented("open-dialog").unwrap().0;
        click(&mut rt, "open-dialog");
        assert!(rt.state.dialog_open);
        close(
            rt.presented("dialog-scrim").unwrap().0,
            [0.0, 0.0, size.0, size.1],
        );
        close(rt.presented("open-dialog").unwrap().0, before);
        rt.set_viewport((800.0, 600.0), 1.0);
        rt.frame(0.0);
        rt.frame(1.0);
        close(
            rt.presented("dialog-scrim").unwrap().0,
            [0.0, 0.0, 800.0, 600.0],
        );
        rt.key(KeyEvent {
            key: Key::Escape,
            mods: Mods::default(),
        });
        rt.frame(0.0);
        assert!(!rt.state.dialog_open);
    }
}
