use rune::{Cx, El, Env, Key, KeyEvent, Mods, Runtime, TextEdit, el};
use rune_material::{
    ADD as PLUS, Badge, Button, ButtonVariant, CHECK, Card, CardVariant, Checkbox, Chip, ChipSet,
    ChipVariant, Dialog, Divider, Fab, FabSize, FabVariant, IconButton, IconButtonVariant, Ink,
    List, ListItem, Menu, MenuItem, Progress, Radio, Scrim, Select, SelectVariant, Slider,
    Snackbar, SnackbarAction, Switch, Tab, TabVariant, Tabs, TextField, TextFieldVariant, Theme,
};

struct State {
    theme: Theme,
    build: fn(&Theme) -> rune::El,
}

fn runtime_with(theme: Theme, build: fn(&Theme) -> rune::El) -> Runtime<State> {
    Runtime::new(
        State { theme, build },
        |s: &State, _: &mut Cx<State>| el().h(300.0).child((s.build)(&s.theme)),
        (400.0, 300.0),
        Env::new(true, 1.0),
    )
}

fn runtime(build: fn(&Theme) -> rune::El) -> Runtime<State> {
    runtime_with(Theme::light(), build)
}

fn info<S>(rt: &Runtime<S>, key: &str) -> rune::ElementInfo {
    rt.elements()
        .into_iter()
        .find(|e| e.key.as_deref() == Some(key))
        .expect("element")
}

fn a11y<'a>(v: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    if v["key"] == key {
        return Some(v);
    }
    v["children"].as_array()?.iter().find_map(|c| a11y(c, key))
}

fn save_button(t: &Theme) -> rune::El {
    Button::new("Save").key("save").build(t)
}

fn variant_buttons(t: &Theme) -> rune::El {
    el().gap(8.0)
        .child(
            Button::new("Filled")
                .variant(ButtonVariant::Filled)
                .key("filled")
                .build(t),
        )
        .child(
            Button::new("Elevated")
                .variant(ButtonVariant::Elevated)
                .key("elevated")
                .build(t),
        )
        .child(
            Button::new("Tonal")
                .variant(ButtonVariant::Tonal)
                .key("tonal")
                .build(t),
        )
        .child(
            Button::new("Outlined")
                .variant(ButtonVariant::Outlined)
                .key("outlined")
                .build(t),
        )
        .child(
            Button::new("Text")
                .variant(ButtonVariant::Text)
                .key("text")
                .build(t),
        )
}

fn disabled_button(t: &Theme) -> rune::El {
    Button::new("Nope").key("b").disabled(true).build(t)
}

fn icon_button(t: &Theme) -> rune::El {
    IconButton::new(&PLUS, "Add").key("add").build(t)
}

fn icon_button_variants(t: &Theme) -> rune::El {
    el().gap(8.0)
        .child(
            IconButton::new(&PLUS, "Tonal")
                .variant(IconButtonVariant::Tonal)
                .key("tonal")
                .build(t),
        )
        .child(
            IconButton::new(&PLUS, "Outlined")
                .variant(IconButtonVariant::Outlined)
                .key("outlined")
                .build(t),
        )
        .child(
            IconButton::new(&PLUS, "Selected")
                .variant(IconButtonVariant::Outlined)
                .selected(Some(true))
                .key("selected")
                .build(t),
        )
}

fn fabs(t: &Theme) -> rune::El {
    el().gap(16.0)
        .child(Fab::new(&PLUS, "New").key("fab").build(t))
        .child(
            Fab::new(&PLUS, "Big")
                .size(FabSize::Large)
                .key("big")
                .build(t),
        )
}

fn fab_variants(t: &Theme) -> rune::El {
    el().gap(16.0)
        .child(
            Fab::new(&PLUS, "Primary")
                .variant(FabVariant::Primary)
                .key("primary")
                .build(t),
        )
        .child(
            Fab::new(&PLUS, "Secondary")
                .variant(FabVariant::Secondary)
                .key("secondary")
                .build(t),
        )
        .child(
            Fab::new(&PLUS, "Tertiary")
                .variant(FabVariant::Tertiary)
                .key("tertiary")
                .build(t),
        )
        .child(
            Fab::new(&PLUS, "Surface")
                .variant(FabVariant::Surface)
                .key("surface")
                .build(t),
        )
}

#[test]
fn filled_button_layout_and_tokens() {
    let mut rt = runtime(save_button);
    rt.frame(0.0);
    let ([_, _, w, h], _) = rt.presented("save").unwrap();
    assert_eq!(h, 40.0);
    assert!(w > 48.0);
    let e = info(&rt, "save");
    assert_eq!(e.fill.hex(), "#6750a4ff");
    assert_eq!(e.role, "button");
    assert_eq!(e.label.as_deref(), Some("Save"));
    assert_eq!(e.radius, 9999.0);
    assert!(e.focusable);
}

#[test]
fn variants_use_material_roles() {
    let theme = Theme::light();
    let mut rt = runtime(variant_buttons);
    rt.frame(0.0);
    assert_eq!(info(&rt, "filled").fill.hex(), theme.color.primary.hex());
    assert_eq!(
        info(&rt, "elevated").fill.hex(),
        theme.color.surface_container_low.hex()
    );
    assert_eq!(
        info(&rt, "tonal").fill.hex(),
        theme.color.secondary_container.hex()
    );
    assert_eq!(info(&rt, "outlined").fill.a, 0.0);
    assert_eq!(info(&rt, "text").fill.a, 0.0);
}

#[test]
fn button_state_layers_match_tokens() {
    let mut rt = runtime(save_button);
    rt.frame(0.0);
    let (x, y) = rt.find("save").unwrap();
    rt.pointer_move(x, y);
    rt.frame(0.5);
    assert!(
        info(&rt, "save").hover == 1.0,
        "hover progress {}",
        info(&rt, "save").hover
    );

    rt.pointer_down(x, y);
    rt.frame(0.5);
    assert!(
        info(&rt, "save").press == 1.0,
        "press progress {}",
        info(&rt, "save").press
    );
}

#[test]
fn disabled_button_states_and_colours() {
    let mut rt = runtime(disabled_button);
    rt.frame(0.0);
    let tree = rt.accessibility_json();
    let node = a11y(&tree, "b").unwrap();
    assert!(
        node["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "disabled")
    );
    let e = info(&rt, "b");
    assert_eq!(e.fill.hex(), "#1d1b201f");
    assert!(!e.focusable);
    assert_eq!(e.hover, 0.0);
}

#[test]
fn icon_button_is_40_square_and_has_24_pt_icon() {
    let mut rt = runtime(icon_button);
    rt.frame(0.0);
    let ([_, _, w, h], _) = rt.presented("add").unwrap();
    assert_eq!((w, h), (40.0, 40.0));
    assert_eq!(info(&rt, "add").fill.hex(), "#6750a4ff");
    assert_eq!(info(&rt, "add").radius, 9999.0);
    assert_eq!(info(&rt, "add").label.as_deref(), Some("Add"));
}

#[test]
fn icon_button_variants_and_toggle_state() {
    let mut rt = runtime(icon_button_variants);
    rt.frame(0.0);
    assert_eq!(info(&rt, "tonal").fill.hex(), "#e8def8ff");
    assert_eq!(info(&rt, "outlined").fill.a, 0.0);
    assert_eq!(info(&rt, "selected").fill.hex(), "#322f35ff");

    let tree = rt.accessibility_json();
    let sel = a11y(&tree, "selected").unwrap();
    assert!(
        sel["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "checked")
    );
}

#[test]
fn fab_is_56_square_primary_container_and_large_is_96() {
    let mut rt = runtime(fabs);
    rt.frame(0.0);
    let ([_, _, w, h], _) = rt.presented("fab").unwrap();
    assert_eq!((w, h), (56.0, 56.0));
    assert_eq!(info(&rt, "fab").fill.hex(), "#eaddffff");
    assert_eq!(info(&rt, "fab").radius, 16.0);
    assert_eq!(info(&rt, "fab").label.as_deref(), Some("New"));

    let ([_, _, w, h], _) = rt.presented("big").unwrap();
    assert_eq!((w, h), (96.0, 96.0));
    assert_eq!(info(&rt, "big").radius, 28.0);
}

#[test]
fn fab_variants_use_material_containers() {
    let theme = Theme::light();
    let mut rt = runtime(fab_variants);
    rt.frame(0.0);
    assert_eq!(
        info(&rt, "primary").fill.hex(),
        theme.color.primary_container.hex()
    );
    assert_eq!(
        info(&rt, "secondary").fill.hex(),
        theme.color.secondary_container.hex()
    );
    assert_eq!(
        info(&rt, "tertiary").fill.hex(),
        theme.color.tertiary_container.hex()
    );
    assert_eq!(
        info(&rt, "surface").fill.hex(),
        theme.color.surface_container_high.hex()
    );
}

#[test]
fn dark_theme_uses_dark_roles() {
    let mut rt = runtime_with(Theme::dark(), save_button);
    rt.frame(0.0);
    assert_eq!(info(&rt, "save").fill.hex(), "#d0bcffff");
}

struct ToggleState {
    theme: Theme,
    checked: bool,
}

fn checkbox_toggle_view(s: &ToggleState, _: &mut Cx<ToggleState>) -> El {
    el().h(300.0).child(
        Checkbox::new(s.checked, "Agree")
            .key("c")
            .build(&s.theme)
            .on_press(|s: &mut ToggleState| s.checked = !s.checked),
    )
}

fn radio_toggle_view(s: &ToggleState, _: &mut Cx<ToggleState>) -> El {
    el().h(300.0).child(
        Radio::new(s.checked, "One")
            .key("r")
            .build(&s.theme)
            .on_press(|s: &mut ToggleState| s.checked = !s.checked),
    )
}

fn switch_toggle_view(s: &ToggleState, _: &mut Cx<ToggleState>) -> El {
    el().h(300.0).child(
        Switch::new(s.checked, "On")
            .key("s")
            .build(&s.theme)
            .on_press(|s: &mut ToggleState| s.checked = !s.checked),
    )
}

fn toggle_runtime(view: fn(&ToggleState, &mut Cx<ToggleState>) -> El) -> Runtime<ToggleState> {
    Runtime::new(
        ToggleState {
            theme: Theme::light(),
            checked: false,
        },
        view,
        (400.0, 300.0),
        Env::new(true, 1.0),
    )
}

fn click_key(rt: &mut Runtime<ToggleState>, key: &str) {
    let (x, y) = rt.find(key).unwrap();
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(0.0);
}

fn key_press(rt: &mut Runtime<ToggleState>, key: Key) {
    rt.key(KeyEvent {
        key,
        mods: Mods::default(),
    });
    rt.frame(0.0);
}

fn unchecked_checkbox(t: &Theme) -> El {
    Checkbox::new(false, "Agree").key("c").build(t)
}

fn checked_checkbox(t: &Theme) -> El {
    Checkbox::new(true, "Agree").key("c").build(t)
}

fn indeterminate_checkbox(t: &Theme) -> El {
    Checkbox::new(false, "Agree")
        .indeterminate(true)
        .key("c")
        .build(t)
}

fn disabled_checkbox(t: &Theme) -> El {
    Checkbox::new(true, "Agree")
        .disabled(true)
        .key("c")
        .build(t)
}

fn error_checkbox(t: &Theme) -> El {
    Checkbox::new(true, "Agree").error(true).key("c").build(t)
}

fn unchecked_radio(t: &Theme) -> El {
    Radio::new(false, "One").key("r").build(t)
}

fn checked_radio(t: &Theme) -> El {
    Radio::new(true, "One").key("r").build(t)
}

fn disabled_radio(t: &Theme) -> El {
    Radio::new(true, "One").disabled(true).key("r").build(t)
}

fn selected_switch(t: &Theme) -> El {
    Switch::new(true, "On").key("s").build(t)
}

fn unselected_switch(t: &Theme) -> El {
    Switch::new(false, "On").key("s").build(t)
}

fn icon_switch(t: &Theme) -> El {
    Switch::new(false, "On").icons(true).key("s").build(t)
}

fn disabled_switch(t: &Theme) -> El {
    Switch::new(true, "On").disabled(true).key("s").build(t)
}

#[test]
fn checkbox_is_18_pt_with_a_2_pt_corner() {
    let theme = Theme::light();
    let mut rt = runtime(unchecked_checkbox);
    rt.frame(0.0);
    let ([_, _, w, h], _) = rt.presented("c").unwrap();
    assert_eq!((w, h), (18.0, 18.0));
    let e = info(&rt, "c");
    assert_eq!(e.radius, 2.0);
    assert_eq!(e.role, "checkbox");
    assert_eq!(e.label.as_deref(), Some("Agree"));
    assert_eq!(e.fill.a, 0.0);
    assert!(e.focusable);

    let tree = rt.accessibility_json();
    let node = a11y(&tree, "c").unwrap();
    assert!(
        node["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "unchecked")
    );
    assert_eq!(theme.color.on_surface_variant.hex(), "#49454fff");
}

#[test]
fn checkbox_selected_indeterminate_and_error_roles() {
    let theme = Theme::light();
    let mut rt = runtime(checked_checkbox);
    rt.frame(0.0);
    assert_eq!(info(&rt, "c").fill.hex(), theme.color.primary.hex());
    assert!(info(&rt, "c").fill.hex() == "#6750a4ff");

    let tree = rt.accessibility_json();
    let node = a11y(&tree, "c").unwrap();
    assert!(
        node["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "checked")
    );

    let mut rt = runtime(indeterminate_checkbox);
    rt.frame(0.0);
    assert_eq!(info(&rt, "c").fill.hex(), theme.color.primary.hex());
    let tree = rt.accessibility_json();
    let node = a11y(&tree, "c").unwrap();
    assert!(
        node["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "mixed")
    );

    let mut rt = runtime(error_checkbox);
    rt.frame(0.0);
    assert_eq!(info(&rt, "c").fill.hex(), theme.color.error.hex());
    assert!(info(&rt, "c").fill.hex() == "#b3261eff");
}

#[test]
fn checkbox_disabled_state_and_overlay() {
    let theme = Theme::light();
    let mut rt = runtime(disabled_checkbox);
    rt.frame(0.0);
    let e = info(&rt, "c");
    assert_eq!(e.fill.hex(), theme.color.on_surface.alpha(0.38).hex());
    assert!(!e.focusable);
    let tree = rt.accessibility_json();
    let node = a11y(&tree, "c").unwrap();
    assert!(
        node["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "disabled")
    );

    rt.pointer_move(e.rect[0] + e.rect[2] * 0.5, e.rect[1] + e.rect[3] * 0.5);
    rt.frame(0.5);
    assert_eq!(info(&rt, "c").hover, 0.0);
}

#[test]
fn checkbox_state_layers_match_tokens() {
    let mut rt = runtime(checked_checkbox);
    rt.frame(0.0);
    let (x, y) = rt.find("c").unwrap();
    rt.pointer_move(x, y);
    rt.frame(0.5);
    assert_eq!(info(&rt, "c").hover, 1.0);
    rt.pointer_down(x, y);
    rt.frame(0.5);
    assert_eq!(info(&rt, "c").press, 1.0);
}

#[test]
fn checkbox_toggles_on_click_and_space() {
    let mut rt = toggle_runtime(checkbox_toggle_view);
    rt.frame(0.0);
    click_key(&mut rt, "c");
    assert!(rt.state.checked);
    click_key(&mut rt, "c");
    assert!(!rt.state.checked);

    key_press(&mut rt, Key::Tab);
    assert!(info(&rt, "c").focused);
    key_press(&mut rt, Key::Text(" ".into()));
    assert!(rt.state.checked);
}

#[test]
fn radio_is_20_pt_and_dot_is_10_pt() {
    let theme = Theme::light();
    let mut rt = runtime(unchecked_radio);
    rt.frame(0.0);
    let ([_, _, w, h], _) = rt.presented("r").unwrap();
    assert_eq!((w, h), (20.0, 20.0));
    let e = info(&rt, "r");
    assert_eq!(e.radius, theme.shape.corner_full);
    assert_eq!(e.role, "radio");
    assert_eq!(e.label.as_deref(), Some("One"));
    assert!(e.focusable);
    assert!(
        rt.elements()
            .iter()
            .all(|e| e.key.as_deref() != Some("dot"))
    );

    let mut rt = runtime(checked_radio);
    rt.frame(0.0);
    let ([_, _, w, h], _) = rt.presented("dot").unwrap();
    assert_eq!((w, h), (10.0, 10.0));
    assert_eq!(info(&rt, "dot").fill.hex(), theme.color.primary.hex());
    assert_eq!(info(&rt, "dot").radius, theme.shape.corner_full);
    let tree = rt.accessibility_json();
    let node = a11y(&tree, "r").unwrap();
    assert!(
        node["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "checked")
    );
}

#[test]
fn radio_disabled_state_and_overlay() {
    let theme = Theme::light();
    let mut rt = runtime(disabled_radio);
    rt.frame(0.0);
    let e = info(&rt, "r");
    assert!(!e.focusable);
    let dot = info(&rt, "dot");
    assert_eq!(dot.fill.hex(), theme.color.on_surface.alpha(0.38).hex());
    let tree = rt.accessibility_json();
    let node = a11y(&tree, "r").unwrap();
    assert!(
        node["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "disabled")
    );

    rt.pointer_move(e.rect[0] + e.rect[2] * 0.5, e.rect[1] + e.rect[3] * 0.5);
    rt.frame(0.5);
    assert_eq!(info(&rt, "r").hover, 0.0);
}

#[test]
fn radio_state_layers_match_tokens() {
    let mut rt = runtime(unchecked_radio);
    rt.frame(0.0);
    let (x, y) = rt.find("r").unwrap();
    rt.pointer_move(x, y);
    rt.frame(0.5);
    assert_eq!(info(&rt, "r").hover, 1.0);
    rt.pointer_down(x, y);
    rt.frame(0.5);
    assert_eq!(info(&rt, "r").press, 1.0);
}

#[test]
fn radio_toggles_on_click_and_space() {
    let mut rt = toggle_runtime(radio_toggle_view);
    rt.frame(0.0);
    click_key(&mut rt, "r");
    assert!(rt.state.checked);
    click_key(&mut rt, "r");
    assert!(!rt.state.checked);

    key_press(&mut rt, Key::Tab);
    assert!(info(&rt, "r").focused);
    key_press(&mut rt, Key::Text(" ".into()));
    assert!(rt.state.checked);
}

#[test]
fn switch_is_52_by_32_with_handles_at_token_positions() {
    let theme = Theme::light();
    let mut rt = runtime(unselected_switch);
    rt.frame(0.0);
    let ([_, _, w, h], _) = rt.presented("s").unwrap();
    assert_eq!((w, h), (52.0, 32.0));
    let e = info(&rt, "s");
    assert_eq!(e.radius, theme.shape.corner_full);
    assert_eq!(e.role, "switch");
    assert_eq!(e.label.as_deref(), Some("On"));
    assert_eq!(e.fill.hex(), theme.color.surface_container_highest.hex());
    assert!(e.focusable);

    let ([x, y, w, h], _) = rt.presented("handle").unwrap();
    assert_eq!((x, y, w, h), (8.0, 8.0, 16.0, 16.0));
    assert_eq!(info(&rt, "handle").fill.hex(), theme.color.outline.hex());

    let mut rt = runtime(selected_switch);
    rt.frame(0.0);
    assert_eq!(info(&rt, "s").fill.hex(), theme.color.primary.hex());
    let ([x, y, w, h], _) = rt.presented("handle").unwrap();
    assert_eq!((x, y, w, h), (24.0, 4.0, 24.0, 24.0));
    assert_eq!(info(&rt, "handle").fill.hex(), theme.color.on_primary.hex());
    let tree = rt.accessibility_json();
    let node = a11y(&tree, "s").unwrap();
    assert!(
        node["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "checked")
    );
}

#[test]
fn switch_icons_and_disabled_roles() {
    let theme = Theme::light();
    let mut rt = runtime(icon_switch);
    rt.frame(0.0);
    let ([x, y, w, h], _) = rt.presented("handle").unwrap();
    assert_eq!((x, y, w, h), (4.0, 4.0, 24.0, 24.0));
    assert_eq!(info(&rt, "handle").fill.hex(), theme.color.outline.hex());

    let mut rt = runtime(disabled_switch);
    rt.frame(0.0);
    let e = info(&rt, "s");
    assert_eq!(e.fill.hex(), theme.color.on_surface.alpha(0.12).hex());
    assert_eq!(info(&rt, "handle").fill.hex(), theme.color.surface.hex());
    assert!(!e.focusable);

    let tree = rt.accessibility_json();
    let node = a11y(&tree, "s").unwrap();
    assert!(
        node["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "disabled")
    );

    rt.pointer_move(e.rect[0] + e.rect[2] * 0.5, e.rect[1] + e.rect[3] * 0.5);
    rt.frame(0.5);
    assert_eq!(info(&rt, "s").hover, 0.0);
}

#[test]
fn switch_state_layers_match_tokens() {
    let mut rt = runtime(selected_switch);
    rt.frame(0.0);
    let (x, y) = rt.find("s").unwrap();
    rt.pointer_move(x, y);
    rt.frame(0.5);
    assert_eq!(info(&rt, "s").hover, 1.0);
    rt.pointer_down(x, y);
    rt.frame(0.5);
    assert_eq!(info(&rt, "s").press, 1.0);
}

#[test]
fn switch_toggles_on_click_and_space() {
    let mut rt = toggle_runtime(switch_toggle_view);
    rt.frame(0.0);
    click_key(&mut rt, "s");
    assert!(rt.state.checked);
    click_key(&mut rt, "s");
    assert!(!rt.state.checked);

    key_press(&mut rt, Key::Tab);
    assert!(info(&rt, "s").focused);
    key_press(&mut rt, Key::Text(" ".into()));
    assert!(rt.state.checked);
}

fn filled_text_field(t: &Theme) -> El {
    TextField::new(&TextEdit::new("Ada"), "Full name")
        .key("tf")
        .width(280.0)
        .build(t)
}

fn outlined_text_field(t: &Theme) -> El {
    TextField::new(&TextEdit::new(""), "Email")
        .variant(TextFieldVariant::Outlined)
        .key("tf")
        .width(280.0)
        .supporting_text("We never share it")
        .build(t)
}

#[test]
fn text_field_is_56_high_with_material_container() {
    let theme = Theme::light();
    let mut rt = runtime(filled_text_field);
    rt.frame(0.0);
    let ([_, _, w, h], _) = rt.presented("tf").unwrap();
    assert_eq!((w, h), (280.0, 56.0));
    let e = info(&rt, "tf");
    assert_eq!(e.role, "textbox");
    assert_eq!(e.label.as_deref(), Some("Full name"));
    assert_eq!(e.fill.hex(), theme.color.surface_container_highest.hex());
    assert!(
        rt.elements()
            .iter()
            .any(|e| e.key.as_deref() == Some("label"))
    );
}

#[test]
fn outlined_text_field_is_transparent_with_4_pt_corners() {
    let theme = Theme::light();
    let mut rt = runtime(outlined_text_field);
    rt.frame(0.0);
    let e = info(&rt, "tf");
    assert_eq!(e.fill.a, 0.0);
    assert_eq!(e.radius, theme.shape.corner_extra_small);
    assert!(
        rt.elements()
            .iter()
            .any(|e| e.key.as_deref() == Some("supporting-text"))
    );
}

struct FieldState {
    theme: Theme,
    text: TextEdit,
}

fn editable_field_view(s: &FieldState, _: &mut Cx<FieldState>) -> El {
    el().h(300.0).child(
        TextField::new(&s.text, "Name")
            .key("tf")
            .width(280.0)
            .build(&s.theme)
            .on_edit(|s: &mut FieldState| &mut s.text),
    )
}

#[test]
fn text_field_routes_edits_through_the_returned_root() {
    let mut rt = Runtime::new(
        FieldState {
            theme: Theme::light(),
            text: TextEdit::new(""),
        },
        editable_field_view,
        (400.0, 300.0),
        Env::new(true, 1.0),
    );
    rt.frame(0.0);
    let (x, y) = rt.find("tf").unwrap();
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(0.0);
    rt.key(KeyEvent {
        key: Key::Text("Hi".into()),
        mods: Mods::default(),
    });
    rt.frame(0.0);
    assert_eq!(rt.state.text.text, "Hi");
}

fn filled_select(t: &Theme) -> El {
    Select::new("Favorite food")
        .value("Pizza")
        .key("sel")
        .width(240.0)
        .build(t)
}

fn outlined_select(t: &Theme) -> El {
    Select::new("Favorite food")
        .variant(SelectVariant::Outlined)
        .value("Pizza")
        .key("sel")
        .width(240.0)
        .build(t)
}

#[test]
fn select_is_56_high_and_at_least_210_wide() {
    let theme = Theme::light();
    let mut rt = runtime(filled_select);
    rt.frame(0.0);
    let ([_, _, w, h], _) = rt.presented("sel").unwrap();
    assert_eq!((w, h), (240.0, 56.0));
    let e = info(&rt, "sel");
    assert_eq!(e.role, "button");
    assert_eq!(e.label.as_deref(), Some("Favorite food"));
    assert_eq!(e.fill.hex(), theme.color.surface_container_highest.hex());
    assert!(e.focusable);
    assert!(
        rt.elements()
            .iter()
            .any(|e| e.key.as_deref() == Some("value"))
    );

    let mut rt = runtime(|t: &Theme| Select::new("Food").key("sel").build(t));
    rt.frame(0.0);
    let ([_, _, w, _], _) = rt.presented("sel").unwrap();
    assert!(w >= 210.0);
}

#[test]
fn outlined_select_is_transparent_with_4_pt_corners() {
    let theme = Theme::light();
    let mut rt = runtime(outlined_select);
    rt.frame(0.0);
    let e = info(&rt, "sel");
    assert_eq!(e.fill.a, 0.0);
    assert_eq!(e.radius, theme.shape.corner_extra_small);
}

fn chips(t: &Theme) -> El {
    ChipSet::new()
        .child(Chip::new("Assist").key("assist").build(t))
        .child(
            Chip::new("Filter")
                .variant(ChipVariant::Filter)
                .selected(true)
                .key("filter")
                .build(t),
        )
        .child(
            Chip::new("Input")
                .variant(ChipVariant::Input)
                .selected(false)
                .key("input")
                .build(t),
        )
        .child(
            Chip::new("Suggestion")
                .variant(ChipVariant::Suggestion)
                .key("suggestion")
                .build(t),
        )
        .build(t)
}

#[test]
fn chips_are_32_high_with_material_chip_roles() {
    let theme = Theme::light();
    let mut rt = runtime(chips);
    rt.frame(0.0);

    for key in ["assist", "filter", "input", "suggestion"] {
        let ([_, _, _, h], _) = rt.presented(key).unwrap();
        assert_eq!(h, 32.0, "{key} height");
        assert_eq!(info(&rt, key).radius, theme.shape.corner_small);
    }

    assert_eq!(info(&rt, "assist").fill.a, 0.0);
    assert_eq!(
        info(&rt, "filter").fill.hex(),
        theme.color.secondary_container.hex()
    );
    assert_eq!(info(&rt, "input").fill.a, 0.0);

    let tree = rt.accessibility_json();
    let filter = a11y(&tree, "filter").unwrap();
    assert!(
        filter["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "checked")
    );
    let input = a11y(&tree, "input").unwrap();
    assert!(
        input["states"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s == "unchecked")
    );
}

fn cards(t: &Theme) -> El {
    el().h(300.0)
        .gap(8.0)
        .child(
            Card::new()
                .variant(CardVariant::Elevated)
                .key("card-elevated")
                .width(240.0)
                .padding(16.0)
                .child(rune::text("Elevated"))
                .build(t),
        )
        .child(
            Card::new()
                .variant(CardVariant::Filled)
                .key("card-filled")
                .width(240.0)
                .build(t),
        )
        .child(
            Card::new()
                .variant(CardVariant::Outlined)
                .key("card-outlined")
                .width(240.0)
                .build(t),
        )
}

#[test]
fn cards_use_material_containers_and_medium_corners() {
    let theme = Theme::light();
    let mut rt = runtime(cards);
    rt.frame(0.0);

    assert_eq!(
        info(&rt, "card-elevated").fill.hex(),
        theme.color.surface_container_low.hex()
    );
    assert_eq!(
        info(&rt, "card-filled").fill.hex(),
        theme.color.surface_container_highest.hex()
    );
    assert_eq!(
        info(&rt, "card-outlined").fill.hex(),
        theme.color.surface.hex()
    );
    for key in ["card-elevated", "card-filled", "card-outlined"] {
        assert_eq!(info(&rt, key).radius, theme.shape.corner_medium, "{key}");
    }
    let ([_, _, w, _], _) = rt.presented("card-elevated").unwrap();
    assert_eq!(w, 240.0);
}

fn dividers(t: &Theme) -> El {
    el().h(100.0)
        .gap(8.0)
        .child(Divider::new().key("divider-h").build(t))
        .child(
            rune::row()
                .h(40.0)
                .center_y()
                .child(Divider::new().vertical(true).key("divider-v").build(t)),
        )
}

#[test]
fn dividers_are_1_pt_outline_variant() {
    let theme = Theme::light();
    let mut rt = runtime(dividers);
    rt.frame(0.0);

    let ([_, _, _, h], _) = rt.presented("divider-h").unwrap();
    assert_eq!(h, 1.0);
    assert_eq!(
        info(&rt, "divider-h").fill.hex(),
        theme.color.outline_variant.hex()
    );
    let ([_, _, w, h], _) = rt.presented("divider-v").unwrap();
    assert_eq!((w, h), (1.0, 40.0));
}

fn badges(t: &Theme) -> El {
    el().gap(8.0)
        .child(Badge::new().key("badge-dot").build(t))
        .child(Badge::new().content("12").key("badge-large").build(t))
}

#[test]
fn badge_is_6_pt_dot_or_16_pt_pill() {
    let theme = Theme::light();
    let mut rt = runtime(badges);
    rt.frame(0.0);

    let ([_, _, w, h], _) = rt.presented("badge-dot").unwrap();
    assert_eq!((w, h), (6.0, 6.0));
    assert_eq!(info(&rt, "badge-dot").fill.hex(), theme.color.error.hex());

    let ([_, _, _, h], _) = rt.presented("badge-large").unwrap();
    assert_eq!(h, 16.0);
    assert_eq!(info(&rt, "badge-large").fill.hex(), theme.color.error.hex());
    assert_eq!(info(&rt, "badge-large").radius, theme.shape.corner_full);
}

fn progress(t: &Theme) -> El {
    Progress::new(0.4).key("progress").width(200.0).build(t)
}

#[test]
fn linear_progress_is_4_pt_with_primary_active_track() {
    let theme = Theme::light();
    let mut rt = runtime(progress);
    rt.frame(0.0);

    let ([_, _, w, h], _) = rt.presented("progress").unwrap();
    assert_eq!((w, h), (200.0, 4.0));
    assert_eq!(
        info(&rt, "progress").fill.hex(),
        theme.color.surface_container_highest.hex()
    );
    let ([_, _, w, h], _) = rt.presented("active").unwrap();
    assert_eq!((w, h), (80.0, 4.0));
    assert_eq!(info(&rt, "active").fill.hex(), theme.color.primary.hex());
}

fn tabs(t: &Theme) -> El {
    Tabs::new()
        .child(Tab::new("One").selected(true).key("tab-one").build(t))
        .child(
            Tab::new("Two")
                .variant(TabVariant::Secondary)
                .selected(false)
                .key("tab-two")
                .build(t),
        )
        .build(t)
        .w(400.0)
}

#[test]
fn tabs_are_48_high_with_3_and_2_pt_indicators() {
    let theme = Theme::light();
    let mut rt = runtime(tabs);
    rt.frame(0.0);

    let ([_, _, w, h], _) = rt.presented("tab-one").unwrap();
    assert_eq!((w, h), (200.0, 48.0));
    let ([_, _, _, h], _) = rt.presented("active-indicator").unwrap();
    assert_eq!(h, 3.0);
    assert_eq!(
        info(&rt, "active-indicator").fill.hex(),
        theme.color.primary.hex()
    );

    assert_eq!(info(&rt, "tab-two").role, "tab");
    assert!(rt.presented("tab-two").is_some());
}

fn list(t: &Theme) -> El {
    List::new()
        .key("list")
        .dividers(true)
        .child(ListItem::new("One").key("item-one").build(t))
        .child(
            ListItem::new("Two")
                .supporting("Support")
                .key("item-two")
                .build(t),
        )
        .child(
            ListItem::new("Three")
                .overline("Overline")
                .supporting("Support")
                .key("item-three")
                .build(t),
        )
        .build(t)
}

#[test]
fn list_items_are_56_72_and_88_high_with_inset_dividers() {
    let mut rt = runtime(list);
    rt.frame(0.0);

    for (key, height) in [("item-one", 56.0), ("item-two", 72.0), ("item-three", 88.0)] {
        let ([_, _, _, h], _) = rt.presented(key).unwrap();
        assert_eq!(h, height, "{key}");
    }
}

fn slider(t: &Theme) -> El {
    Slider::new(0.35).key("slider").width(280.0).build(t)
}

#[test]
fn slider_has_4_pt_tracks_and_a_20_pt_handle() {
    let theme = Theme::light();
    let mut rt = runtime(slider);
    rt.frame(0.0);

    let ([_, _, w, h], _) = rt.presented("slider").unwrap();
    assert_eq!((w, h), (280.0, 40.0));
    let ([_, _, w, h], _) = rt.presented("active-track").unwrap();
    assert_eq!((w, h), (91.0, 4.0));
    let ([_, _, w, h], _) = rt.presented("handle").unwrap();
    assert_eq!((w, h), (20.0, 20.0));
    assert_eq!(info(&rt, "handle").fill.hex(), theme.color.primary.hex());
    let ([_, _, w, h], _) = rt.presented("state-layer").unwrap();
    assert_eq!((w, h), (40.0, 40.0));
}

fn dialog(t: &Theme) -> El {
    Dialog::new()
        .key("dialog")
        .headline("Reset settings?")
        .supporting_text("This restores the baseline theme.")
        .width(320.0)
        .action(Button::new("Cancel").variant(ButtonVariant::Text).build(t))
        .build(t)
}

#[test]
fn dialog_uses_surface_container_high_and_extra_large_corners() {
    let theme = Theme::light();
    let mut rt = runtime(dialog);
    rt.frame(0.0);

    let ([_, _, w, _], _) = rt.presented("dialog").unwrap();
    assert_eq!(w, 320.0);
    let e = info(&rt, "dialog");
    assert_eq!(e.role, "dialog");
    assert_eq!(e.label.as_deref(), Some("Reset settings?"));
    assert_eq!(e.fill.hex(), theme.color.surface_container_high.hex());
    assert_eq!(e.radius, theme.shape.corner_extra_large);
}

fn scrim(t: &Theme) -> El {
    Scrim::new().key("scrim").build(t)
}

#[test]
fn scrim_is_scrim_at_32_percent() {
    let theme = Theme::light();
    let mut rt = runtime(scrim);
    rt.frame(0.0);

    let e = info(&rt, "scrim");
    assert_eq!(e.fill.r, theme.color.scrim.r);
    assert!((e.fill.a - 0.32).abs() < 0.001);
    let ([_, _, w, h], _) = rt.presented("scrim").unwrap();
    assert_eq!((w, h), (400.0, 300.0));
}

fn snackbar(t: &Theme) -> El {
    rune::col().child(
        Snackbar::new("Item archived")
            .key("snackbar")
            .width(320.0)
            .action(SnackbarAction::new("Undo").key("snackbar-action").build(t))
            .build(t),
    )
}

fn snackbar_two_line(t: &Theme) -> El {
    rune::col().child(
        Snackbar::new("A two-line snackbar can hold a longer message, with its action at the end")
            .key("snackbar")
            .width(320.0)
            .two_line(true)
            .action(SnackbarAction::new("Dismiss").build(t))
            .build(t),
    )
}

#[test]
fn snackbar_is_48_high_and_uses_inverse_surface() {
    let theme = Theme::light();
    let mut rt = runtime(snackbar);
    rt.frame(0.0);

    let ([_, _, w, h], _) = rt.presented("snackbar").unwrap();
    assert_eq!((w, h), (320.0, 48.0));
    assert_eq!(
        info(&rt, "snackbar").fill.hex(),
        theme.color.inverse_surface.hex()
    );
    assert_eq!(info(&rt, "snackbar").radius, theme.shape.corner_extra_small);
    let ([_, _, _, h], _) = rt.presented("snackbar-action").unwrap();
    assert_eq!(h, 40.0);
}

#[test]
fn two_line_snackbar_is_at_least_68_high_and_wraps() {
    let mut rt = runtime(snackbar_two_line);
    rt.frame(0.0);

    let ([_, _, _, h], _) = rt.presented("snackbar").unwrap();
    assert!(h >= 68.0, "height {h}");
}

fn menu(t: &Theme) -> El {
    Menu::new()
        .key("menu")
        .width(160.0)
        .child(
            MenuItem::new("Pizza")
                .selected(true)
                .key("menu-pizza")
                .build(t),
        )
        .child(MenuItem::new("Pasta").key("menu-pasta").build(t))
        .build(t)
}

#[test]
fn menu_uses_surface_container_and_selected_items_use_secondary_container() {
    let theme = Theme::light();
    let mut rt = runtime(menu);
    rt.frame(0.0);

    let ([_, _, w, _], _) = rt.presented("menu").unwrap();
    assert_eq!(w, 160.0);
    assert_eq!(
        info(&rt, "menu").fill.hex(),
        theme.color.surface_container.hex()
    );
    assert_eq!(info(&rt, "menu").radius, theme.shape.corner_extra_small);
    assert_eq!(
        info(&rt, "menu-pizza").fill.hex(),
        theme.color.secondary_container.hex()
    );
    assert_eq!(info(&rt, "menu-pizza").role, "menuitem");
    let ([_, _, _, h], _) = rt.presented("menu-pasta").unwrap();
    assert_eq!(h, 56.0);
}

struct InkState {
    theme: Theme,
    ink: Ink,
    pressed: bool,
}

fn ink_button_view(s: &InkState, _: &mut Cx<InkState>) -> El {
    el().h(300.0).child(
        Button::new("Save")
            .key("save")
            .ink(&s.ink)
            .build(&s.theme)
            .on_press(|s: &mut InkState| s.pressed = true),
    )
}

#[test]
fn ink_ripple_spreads_from_the_press_point_and_expires() {
    let mut rt = Runtime::new(
        InkState {
            theme: Theme::light(),
            ink: Ink::new(),
            pressed: false,
        },
        ink_button_view,
        (400.0, 300.0),
        Env::new(true, 1.0),
    );
    rt.poll = Some(|s: &mut InkState, now, _| s.ink.poll(now));
    rt.frame(0.0);
    assert!(rt.state.ink.is_empty());

    let (x, y) = rt.find("save").unwrap();
    rt.pointer_down(x, y);
    rt.frame(0.0);

    let ripples = rt.state.ink.ripples("save");
    assert_eq!(ripples.len(), 1);
    let circle = format!("ink-save-{}", ripples[0].id);
    // The wash layer itself is transition-free, so the circle plays its own
    // scale entrance instead of riding a layer fade: half size at mount,
    // settling to full while held.
    let ([_, _, w0, _], _) = rt.presented(&circle).expect("ripple circle mounted");
    assert!(
        w0 < 2.0 * ripples[0].r * 0.75,
        "enter starts small, got width {w0}"
    );
    let ([bx, by, bw, bh], _) = rt.presented("save").unwrap();
    // The wash layer stretch-fills the button.
    assert_eq!(rt.presented("inklayer-save").unwrap().0, [bx, by, bw, bh]);
    // Let the enter spring settle while held: held ripples do not expire.
    // `presented` reports in-transition bounds mid-enter, so geometry is only
    // asserted once settled.
    rt.frame(0.1);
    rt.frame(0.1);
    rt.frame(0.1);
    assert_eq!(
        rt.state.ink.ripples("save").len(),
        1,
        "held press stays mounted"
    );
    let ([cx, cy, w, h], _) = rt.presented(&circle).expect("ripple circle mounted");
    // Circle centred on the press point, sized to the farthest corner.
    assert!((cx + w / 2.0 - x).abs() < 1.5, "center_x {}", cx + w / 2.0);
    assert!((cy + h / 2.0 - y).abs() < 1.5, "center_y {}", cy + h / 2.0);
    assert!((w - 2.0 * ripples[0].r).abs() < 1.5, "diameter {w}");
    assert!((h - 2.0 * ripples[0].r).abs() < 1.5, "diameter {h}");
    // Fill is the filled variant's pressed state layer.
    let theme = Theme::light();
    assert_eq!(
        info(&rt, &circle).fill.hex(),
        theme.color.on_primary.alpha(0.12).hex()
    );

    // Release fades the ripple out through the wired poll. The release and
    // the reap land on different frames, so step past the fade twice.
    rt.pointer_up(x, y);
    rt.frame(0.5);
    assert!(
        rt.state.pressed,
        "app on_press still fires with ink capture attached"
    );
    rt.frame(0.5);
    assert!(rt.state.ink.is_empty());
}

fn ink_card_view(s: &InkState, _: &mut Cx<InkState>) -> El {
    el().h(300.0).child(
        Card::new()
            .key("card")
            .width(240.0)
            .padding(16.0)
            .ink(&s.ink)
            .child(rune::text("Pressable"))
            .build(&s.theme)
            .on_press(|s: &mut InkState| s.pressed = true),
    )
}

#[test]
fn cards_ripple_when_pressed() {
    let mut rt = Runtime::new(
        InkState {
            theme: Theme::light(),
            ink: Ink::new(),
            pressed: false,
        },
        ink_card_view,
        (400.0, 300.0),
        Env::new(true, 1.0),
    );
    rt.poll = Some(|s: &mut InkState, now, _| s.ink.poll(now));
    rt.frame(0.0);

    let (x, y) = rt.find("card").unwrap();
    rt.pointer_down(x, y);
    rt.frame(0.0);
    assert_eq!(rt.state.ink.ripples("card").len(), 1);
    assert!(rt.presented("ink-card-0").is_some());

    rt.pointer_up(x, y);
    rt.frame(0.5);
    assert!(rt.state.pressed);
    rt.frame(0.5);
    assert!(rt.state.ink.is_empty());
}

#[test]
fn buttons_without_ink_paint_no_wash_layer() {
    let mut rt = runtime(save_button);
    rt.frame(0.0);
    let (x, y) = rt.find("save").unwrap();
    rt.pointer_down(x, y);
    rt.frame(0.0);
    assert!(
        rt.elements()
            .iter()
            .all(|e| !e.key.as_deref().unwrap_or("").starts_with("ink-")),
        "no ink circles without .ink()"
    );
}

struct ChipInkState {
    theme: Theme,
    ink: Ink,
}

fn chip_ink_view(s: &ChipInkState, _: &mut Cx<ChipInkState>) -> El {
    el().h(300.0).child(
        Chip::new("Assist")
            .key("chip")
            .icon(&CHECK)
            .ink(&s.ink)
            .build(&s.theme),
    )
}

#[test]
fn chip_content_keeps_identity_when_ripple_mounts() {
    let mut rt = Runtime::new(
        ChipInkState {
            theme: Theme::light(),
            ink: Ink::new(),
        },
        chip_ink_view,
        (400.0, 300.0),
        Env::new(true, 1.0),
    );
    rt.poll = Some(|s: &mut ChipInkState, now, _| s.ink.poll(now));
    rt.frame(0.0);

    // Content carries derived keys so the wash layer joining child 0 cannot
    // renumber it into new identities (which would replay enter transitions
    // mid-press: the duplicated sliding icon).
    let icon_before = info_by_path(&rt, "#0/chip/chip:icon");
    let label_before = info_by_path(&rt, "#0/chip/chip:label");
    assert_eq!(icon_before.rect, [8.0, 7.0, 18.0, 18.0]);

    let (x, y) = rt.find("chip").unwrap();
    rt.pointer_down(x, y);
    rt.frame(0.0);
    assert_eq!(rt.state.ink.ripples("chip").len(), 1);

    // Same paths, no exits, fully opaque, unmoved: no remount happened.
    for (path, before) in [
        ("#0/chip/chip:icon", icon_before),
        ("#0/chip/chip:label", label_before),
    ] {
        let after = info_by_path(&rt, path);
        assert_eq!(after.rect, before.rect, "{path} moved");
        assert_eq!(after.opacity, 1.0, "{path} faded");
        assert!(!after.exiting, "{path} exiting");
    }
    assert!(
        rt.elements().iter().all(|e| !e.exiting),
        "nothing else is exiting either"
    );

    rt.pointer_up(x, y);
    rt.frame(0.5);
    rt.frame(0.5);
    assert!(rt.state.ink.is_empty());
}

fn info_by_path<S>(rt: &Runtime<S>, path: &str) -> rune::ElementInfo {
    rt.elements()
        .into_iter()
        .find(|e| e.path == path)
        .unwrap_or_else(|| panic!("no element at {path}"))
}

struct ShareState {
    theme: Theme,
    text: TextEdit,
    error_text: TextEdit,
}

fn shared_fields_view(s: &ShareState, _: &mut Cx<ShareState>) -> El {
    el().h(300.0)
        .w(400.0)
        .child(
            rune_material::TextField::new(&s.text, "Email")
                .key("email")
                .width(240.0)
                .build(&s.theme)
                .on_edit(|s: &mut ShareState| &mut s.text),
        )
        .child(
            rune_material::TextField::new(&s.error_text, "Email")
                .key("email-error")
                .width(240.0)
                .error(true)
                .build(&s.theme),
        )
}

fn focused_key<S>(rt: &Runtime<S>) -> Option<String> {
    rt.elements()
        .into_iter()
        .find(|e| e.focused)
        .and_then(|e| e.key)
}

#[test]
fn each_live_input_needs_its_own_textedit_model() {
    // Two inputs sharing one TextEdit flap focus: after the click commit the
    // display-only twin counts as newly requested (same model, never focused
    // before) and steals it. Separate models hold focus across edits.
    let mut rt = Runtime::new(
        ShareState {
            theme: Theme::light(),
            text: TextEdit::new("x"),
            error_text: TextEdit::new("x"),
        },
        shared_fields_view,
        (400.0, 300.0),
        Env::new(true, 1.0),
    );
    rt.frame(0.0);
    let (x, y) = rt.find("email").unwrap();
    rt.pointer_down(x, y);
    rt.pointer_up(x, y);
    rt.frame(0.0);
    assert_eq!(focused_key(&rt).as_deref(), Some("email"));
    for key in ["H", "i"] {
        rt.key(KeyEvent {
            key: Key::Text(key.into()),
            mods: Mods::default(),
        });
        rt.frame(0.0);
        assert_eq!(
            focused_key(&rt).as_deref(),
            Some("email"),
            "focus must stay through edits",
        );
    }
    assert_eq!(rt.state.text.text, "xHi");
    assert_eq!(rt.state.error_text.text, "x", "display model untouched");
}
