//! Widget gallery for rune-material. Run it, or render light/dark snapshots with
//! `cargo run --bin gallery -- --snap light.png --scene light`.

use rune::{AppSpec, Cx, El, ScrollSpec, TextEdit, col, el, row, text};
use rune_material::{
    ADD as PLUS, Badge, Button, ButtonVariant, CHECK, CLOSE, Card, CardVariant, Checkbox, Chip,
    ChipSet, ChipVariant, Dialog, Divider, Fab, FabSize, FabVariant, IconButton, IconButtonVariant,
    Ink, List, ListItem, Menu, MenuItem, Progress, Radio, SEARCH, Scrim, Select, SelectVariant,
    Slider, Snackbar, SnackbarAction, Switch, Tab, TabVariant, Tabs, TextField, TextFieldVariant,
    Theme,
};

#[cfg(test)]
mod tests;

struct State {
    theme: Theme,
    fonts: rune_material::FontScale,
    ink: Ink,
    checked: bool,
    radio: bool,
    switch_on: bool,
    name: TextEdit,
    email: TextEdit,
    // Display-only twins of the fields above. Each live input needs its own
    // TextEdit model: sharing one makes focus flap between them, because the
    // framework recognises a field across rebuilds by its model.
    email_error_text: TextEdit,
    name_disabled_text: TextEdit,
    food: String,
    select_open: bool,
    filter_a: bool,
    filter_b: bool,
    input_chip: bool,
    tab: usize,
    secondary_tab: usize,
    slider: f32,
    list_selected: usize,
    dialog_open: bool,
    menu_open: bool,
    message: String,
}

fn init() -> State {
    let fonts = rune_material::FontScale::from_desktop();
    let mut theme = Theme::light();
    fonts.apply(&mut theme);
    State {
        theme,
        fonts,
        ink: Ink::new(),
        checked: false,
        radio: false,
        switch_on: false,
        name: TextEdit::new("Ada Lovelace"),
        email: TextEdit::new("ada@example"),
        email_error_text: TextEdit::new("ada@example"),
        name_disabled_text: TextEdit::new("Ada Lovelace"),
        food: "Pizza".into(),
        select_open: false,
        filter_a: true,
        filter_b: false,
        input_chip: true,
        tab: 0,
        secondary_tab: 1,
        slider: 0.35,
        list_selected: 0,
        dialog_open: false,
        menu_open: false,
        message: "Ready".into(),
    }
}

fn scene_light(s: &mut State) {
    s.fonts = Default::default();
    s.theme = Theme::light();
}

fn scene_dark(s: &mut State) {
    s.fonts = Default::default();
    s.theme = Theme::dark();
}

fn scene_dialog(s: &mut State) {
    s.fonts = Default::default();
    s.theme = Theme::light();
    s.dialog_open = true;
}

fn scene_menu(s: &mut State) {
    s.fonts = Default::default();
    s.theme = Theme::light();
    s.menu_open = true;
}

fn scene_ink(s: &mut State) {
    s.fonts = Default::default();
    s.theme = Theme::light();
    // A live ripple on the filled button: well inside any button geometry, so
    // the snapshot shows the wash clipped to the pill.
    s.ink.press_at("filled", 20.0, 20.0, 200.0, 40.0, 0.0);
}

type Scene = (&'static str, fn(&mut State));
const SCENES: &[Scene] = &[
    ("light", scene_light),
    ("dark", scene_dark),
    ("dialog", scene_dialog),
    ("menu", scene_menu),
    ("ink", scene_ink),
];

fn caption(t: &Theme, s: impl Into<String>) -> El {
    text(s)
        .font(t.type_scale.body_medium)
        .color(t.color.on_surface_variant)
}

fn title(t: &Theme, s: impl Into<String>) -> El {
    text(s)
        .font(t.type_scale.title_medium)
        .color(t.color.on_surface)
}

fn section(t: &Theme, name: &str, body: El) -> El {
    col().gap(12.0).child(title(t, name)).child(body)
}

fn actions(s: &State) -> El {
    let t = &s.theme;

    let buttons = row()
        .gap(8.0)
        .center_y()
        .child(
            Button::new("Filled")
                .key("filled")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.message = "Filled pressed".into()),
        )
        .child(
            Button::new("Elevated")
                .variant(ButtonVariant::Elevated)
                .key("elevated")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.message = "Elevated pressed".into()),
        )
        .child(
            Button::new("Tonal")
                .variant(ButtonVariant::Tonal)
                .key("tonal")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.message = "Tonal pressed".into()),
        )
        .child(
            Button::new("Outlined")
                .variant(ButtonVariant::Outlined)
                .key("outlined")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.message = "Outlined pressed".into()),
        )
        .child(
            Button::new("Text")
                .variant(ButtonVariant::Text)
                .key("text")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.message = "Text pressed".into()),
        )
        .child(
            Button::new("Disabled")
                .key("disabled")
                .disabled(true)
                .ink(&s.ink)
                .build(t),
        );

    let icon_buttons = row()
        .gap(8.0)
        .center_y()
        .child(
            IconButton::new(&PLUS, "Add")
                .key("icon-filled")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.message = "Icon pressed".into()),
        )
        .child(
            IconButton::new(&PLUS, "Add tonal")
                .variant(IconButtonVariant::Tonal)
                .key("icon-tonal")
                .ink(&s.ink)
                .build(t),
        )
        .child(
            IconButton::new(&PLUS, "Add outlined")
                .variant(IconButtonVariant::Outlined)
                .key("icon-outlined")
                .ink(&s.ink)
                .build(t),
        )
        .child(
            IconButton::new(&PLUS, "Add disabled")
                .disabled(true)
                .key("icon-disabled")
                .ink(&s.ink)
                .build(t),
        );

    let fabs = row()
        .gap(16.0)
        .center_y()
        .child(
            Fab::new(&PLUS, "New")
                .key("fab")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.message = "FAB pressed".into()),
        )
        .child(
            Fab::new(&PLUS, "Small")
                .size(FabSize::Small)
                .key("fab-small")
                .ink(&s.ink)
                .build(t),
        )
        .child(
            Fab::new(&PLUS, "Large")
                .size(FabSize::Large)
                .key("fab-large")
                .ink(&s.ink)
                .build(t),
        )
        .child(
            Fab::new(&PLUS, "Surface")
                .variant(FabVariant::Surface)
                .key("fab-surface")
                .ink(&s.ink)
                .build(t),
        );

    col()
        .gap(16.0)
        .child(buttons)
        .child(icon_buttons)
        .child(fabs)
        .child(caption(t, "Buttons, icon buttons and FABs"))
}

fn selection(s: &State) -> El {
    let t = &s.theme;

    let checkbox = row()
        .center_y()
        .gap(8.0)
        .child(
            Checkbox::new(s.checked, "Accept terms")
                .key("checkbox")
                .build(t)
                .on_press(|s: &mut State| s.checked = !s.checked),
        )
        .child(caption(t, "Checkbox"));

    let radio = row()
        .center_y()
        .gap(8.0)
        .child(
            Radio::new(s.radio, "Option one")
                .key("radio")
                .build(t)
                .on_press(|s: &mut State| s.radio = !s.radio),
        )
        .child(caption(t, "Radio"));

    let switch = row()
        .center_y()
        .gap(8.0)
        .child(
            Switch::new(s.switch_on, "Notifications")
                .key("switch")
                .build(t)
                .on_press(|s: &mut State| s.switch_on = !s.switch_on),
        )
        .child(caption(t, "Switch"));

    let states = row()
        .gap(24.0)
        .center_y()
        .child(Checkbox::new(false, "Unchecked").key("unchecked").build(t))
        .child(Checkbox::new(true, "Checked").key("checked").build(t))
        .child(
            Checkbox::new(false, "Mixed")
                .indeterminate(true)
                .key("mixed")
                .build(t),
        )
        .child(
            Checkbox::new(true, "Disabled")
                .disabled(true)
                .key("selection-disabled")
                .build(t),
        );

    col()
        .gap(16.0)
        .child(checkbox)
        .child(radio)
        .child(switch)
        .child(states)
        .child(caption(
            t,
            "Selection controls, including disabled and indeterminate states",
        ))
}

fn fields(s: &State) -> El {
    let t = &s.theme;

    let fields = row()
        .gap(24.0)
        .flex_wrap()
        .items_start()
        .child(
            TextField::new(&s.name, "Full name")
                .key("name")
                .width(280.0)
                .leading_icon(&SEARCH)
                .build(t)
                .on_edit(|s: &mut State| &mut s.name),
        )
        .child(
            TextField::new(&s.email, "Email")
                .variant(TextFieldVariant::Outlined)
                .key("email")
                .width(280.0)
                .supporting_text("We never share it")
                .build(t)
                .on_edit(|s: &mut State| &mut s.email),
        )
        .child(
            TextField::new(&s.email_error_text, "Email")
                .variant(TextFieldVariant::Outlined)
                .key("email-error")
                .width(280.0)
                .error(true)
                .supporting_text("Enter a valid address")
                .build(t),
        )
        .child(
            TextField::new(&s.name_disabled_text, "Disabled")
                .key("name-disabled")
                .width(280.0)
                .disabled(true)
                .build(t),
        );

    let selects = row()
        .gap(24.0)
        .flex_wrap()
        .items_start()
        .child(
            Select::new("Favorite food")
                .key("select")
                .value(&s.food)
                .open(s.select_open)
                .width(240.0)
                .build(t)
                .on_press(|s: &mut State| s.select_open = !s.select_open),
        )
        .child(
            Select::new("Favorite food")
                .variant(SelectVariant::Outlined)
                .key("select-outlined")
                .value(&s.food)
                .width(240.0)
                .build(t)
                .on_press(|s: &mut State| s.message = "Select pressed".into()),
        )
        .child(
            Select::new("Disabled")
                .key("select-disabled")
                .value("Locked")
                .width(240.0)
                .disabled(true)
                .build(t),
        );

    col().gap(24.0).child(fields).child(selects).child(caption(
        t,
        "Text fields and selects, including outlined, error, supporting and disabled states",
    ))
}

fn chips(s: &State) -> El {
    let t = &s.theme;

    let set = ChipSet::new()
        .key("chip-set")
        .child(
            Chip::new("Assist")
                .key("chip-assist")
                .icon(&CHECK)
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.message = "Assist chip".into()),
        )
        .child(
            Chip::new("Elevated")
                .key("chip-elevated")
                .elevated(true)
                .ink(&s.ink)
                .build(t),
        )
        .child(
            Chip::new("Filter")
                .variant(ChipVariant::Filter)
                .selected(s.filter_a)
                .key("chip-filter-a")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.filter_a = !s.filter_a),
        )
        .child(
            Chip::new("Another")
                .variant(ChipVariant::Filter)
                .selected(s.filter_b)
                .key("chip-filter-b")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.filter_b = !s.filter_b),
        )
        .child(
            Chip::new("Input")
                .variant(ChipVariant::Input)
                .selected(s.input_chip)
                .key("chip-input")
                .trailing_icon(&CLOSE)
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.input_chip = !s.input_chip),
        )
        .child(
            Chip::new("Suggestion")
                .variant(ChipVariant::Suggestion)
                .key("chip-suggestion")
                .icon(&SEARCH)
                .ink(&s.ink)
                .build(t),
        )
        .build(t);

    col().gap(16.0).child(set).child(caption(
        t,
        "Assist, elevated, filter, input and suggestion chips in an 8 pt chip set",
    ))
}

fn cards(s: &State) -> El {
    let t = &s.theme;

    let card = |variant: CardVariant, title: &str, body: &str, key: &'static str| {
        let message = format!("{title} card");
        // One static key per card: ripples match by key, so the three action
        // buttons must not share one.
        let action_key = match variant {
            CardVariant::Elevated => "card-action-elevated",
            CardVariant::Filled => "card-action-filled",
            CardVariant::Outlined => "card-action-outlined",
        };
        Card::new()
            .variant(variant)
            .key(key)
            .width(240.0)
            .padding(16.0)
            .child(
                col()
                    .w_full()
                    .gap(8.0)
                    .child(
                        text(title.to_owned())
                            .font(t.type_scale.title_medium)
                            .color(t.color.on_surface),
                    )
                    .child(
                        text(body.to_owned())
                            .font(t.type_scale.body_medium)
                            .color(t.color.on_surface_variant)
                            .wrap(),
                    )
                    .child(
                        Button::new("Action")
                            .variant(ButtonVariant::Text)
                            .key(action_key)
                            .ink(&s.ink)
                            .build(t)
                            .on_press(move |s: &mut State| s.message = message.clone()),
                    ),
            )
            .build(t)
    };

    col()
        .gap(16.0)
        .child(
            row()
                .gap(16.0)
                .flex_wrap()
                .items_start()
                .child(card(
                    CardVariant::Elevated,
                    "Elevated",
                    "surface-container-low at level 1",
                    "card-elevated",
                ))
                .child(card(
                    CardVariant::Filled,
                    "Filled",
                    "surface-container-highest at level 0",
                    "card-filled",
                ))
                .child(card(
                    CardVariant::Outlined,
                    "Outlined",
                    "surface with a 1 pt outline-variant border",
                    "card-outlined",
                )),
        )
        .child(caption(
            t,
            "Elevated, filled and outlined cards with 12 pt corners",
        ))
}

fn indicators(s: &State) -> El {
    let t = &s.theme;

    row()
        .gap(32.0)
        .center_y()
        .child(
            row()
                .gap(8.0)
                .center_y()
                .child(Badge::new().key("badge-dot").build(t))
                .child(Badge::new().content("12").key("badge-large").build(t)),
        )
        .child(
            col()
                .gap(12.0)
                .child(
                    Progress::new(0.4)
                        .key("progress-40")
                        .width(200.0)
                        .label("40 percent")
                        .build(t),
                )
                .child(Progress::new(0.75).key("progress-75").width(200.0).build(t)),
        )
        .child(
            row()
                .gap(12.0)
                .center_y()
                .child(
                    text("Before")
                        .font(t.type_scale.body_medium)
                        .color(t.color.on_surface_variant),
                )
                .child(
                    Divider::new()
                        .vertical(true)
                        .inset(4.0)
                        .key("divider-v")
                        .build(t),
                )
                .child(
                    text("After")
                        .font(t.type_scale.body_medium)
                        .color(t.color.on_surface_variant),
                ),
        )
}

fn tabs_section(s: &State) -> El {
    let t = &s.theme;

    let primary = Tabs::new()
        .key("tabs-primary")
        .child(
            Tab::new("Overview")
                .key("tab-overview")
                .selected(s.tab == 0)
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.tab = 0),
        )
        .child(
            Tab::new("Details")
                .key("tab-details")
                .selected(s.tab == 1)
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.tab = 1),
        )
        .child(
            Tab::new("Settings")
                .key("tab-settings")
                .selected(s.tab == 2)
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.tab = 2),
        )
        .build(t);

    let secondary = Tabs::new()
        .key("tabs-secondary")
        .child(
            Tab::new("Music")
                .variant(TabVariant::Secondary)
                .key("tab-music")
                .selected(s.secondary_tab == 0)
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.secondary_tab = 0),
        )
        .child(
            Tab::new("Podcasts")
                .variant(TabVariant::Secondary)
                .key("tab-podcasts")
                .selected(s.secondary_tab == 1)
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.secondary_tab = 1),
        )
        .build(t);

    col()
        .w(520.0)
        .gap(24.0)
        .child(primary)
        .child(secondary)
        .child(caption(
            t,
            "Primary tabs with a 3 pt indicator and secondary tabs with a 2 pt indicator",
        ))
}

fn lists(s: &State) -> El {
    let t = &s.theme;

    let item = |index: usize, headline: &str, supporting: &str, key: &'static str| {
        ListItem::new(headline)
            .key(key)
            .supporting(supporting)
            .leading_icon(&CHECK)
            .trailing_icon(&CHECK)
            .selected(s.list_selected == index)
            .ink(&s.ink)
            .build(t)
            .on_press(move |s: &mut State| s.list_selected = index)
    };

    List::new()
        .key("list")
        .dividers(true)
        .child(item(0, "First item", "One-line support", "list-1"))
        .child(item(1, "Second item", "Two lines at 72 pt", "list-2"))
        .child(item(2, "Third item", "Selected trailing icon", "list-3"))
        .build(t)
        .w(440.0)
}

fn slider_section(s: &State) -> El {
    let t = &s.theme;

    col()
        .gap(12.0)
        .child(
            Slider::new(s.slider)
                .key("slider")
                .width(280.0)
                .label("Volume")
                .build(t)
                .on_drag(|s: &mut State, d: &mut rune::Drag, _tx: &mut rune::Tx| {
                    s.slider = ((d.x - 10.0) / 260.0).clamp(0.0, 1.0);
                }),
        )
        .child(caption(t, format!("Slider value {:.0}%", s.slider * 100.0)))
}

fn snackbar_section(s: &State) -> El {
    let t = &s.theme;

    let action = SnackbarAction::new("Undo")
        .key("snackbar-action")
        .ink(&s.ink)
        .build(t)
        .on_press(|s: &mut State| s.message = "Undone".into());

    col()
        .gap(16.0)
        .child(
            Snackbar::new("Item archived")
                .key("snackbar")
                .width(440.0)
                .action(action)
                .build(t),
        )
        .child(
            Snackbar::new(
                "A two-line snackbar can hold a longer message, with its action at the end",
            )
            .key("snackbar-two-line")
            .width(440.0)
            .two_line(true)
            .action(
                SnackbarAction::new("Dismiss")
                    .key("snackbar-dismiss")
                    .ink(&s.ink)
                    .build(t)
                    .on_press(|s: &mut State| s.message = "Dismissed".into()),
            )
            .build(t),
        )
}

fn overlay_demo(s: &State) -> El {
    let t = &s.theme;

    col()
        .gap(12.0)
        .child(
            row()
                .gap(8.0)
                .center_y()
                .child(
                    Button::new("Open dialog")
                        .key("open-dialog")
                        .ink(&s.ink)
                        .build(t)
                        .on_press(|s: &mut State| s.dialog_open = true),
                )
                .child(
                    Button::new("Open menu")
                        .variant(ButtonVariant::Outlined)
                        .key("open-menu")
                        .ink(&s.ink)
                        .build(t)
                        .on_layout_tx(|s: &mut State, _, tx: &mut rune::Tx| {
                            // Keep an open popup's trigger visible after a
                            // resize, and in the pre-opened menu scene. This
                            // changes only scroll state, never the view model.
                            if s.menu_open {
                                tx.scroll_into_view_in("gallery-scroll", "open-menu");
                                tx.instant();
                            }
                            tx.live_only();
                        })
                        .on_press(|s: &mut State| s.menu_open = true),
                ),
        )
        .child(caption(
            t,
            "Dialog and menu surfaces mounted as app-owned overlay layers",
        ))
}

fn theme_controls(s: &State) -> El {
    let t = &s.theme;

    row()
        .gap(8.0)
        .center_y()
        .child(
            Button::new("Light")
                .variant(ButtonVariant::Outlined)
                .key("theme-light")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.theme = Theme::light()),
        )
        .child(
            Button::new("Dark")
                .variant(ButtonVariant::Tonal)
                .key("theme-dark")
                .ink(&s.ink)
                .build(t)
                .on_press(|s: &mut State| s.theme = Theme::dark()),
        )
        .child(caption(
            t,
            if s.theme.dark {
                "Dark baseline"
            } else {
                "Light baseline"
            },
        ))
}

fn overlays(s: &State) -> Vec<El> {
    let t = &s.theme;
    let mut layers = Vec::new();

    if s.dialog_open {
        layers.push(
            Scrim::new()
                .key("dialog-scrim")
                .backdrop_blur(8.0)
                .build(t)
                // Out of flow: neither reserve column space nor shrink the
                // scroll viewport when the full-window backdrop appears.
                .pin(Some(0.0), Some(0.0), Some(0.0), Some(0.0)),
        );
        let dialog = Dialog::new()
            .key("dialog")
            .headline("Reset settings?")
            .supporting_text("This restores the gallery to its baseline theme and state.")
            .action(
                Button::new("Cancel")
                    .variant(ButtonVariant::Text)
                    .key("dialog-cancel")
                    .ink(&s.ink)
                    .build(t)
                    .on_press(|s: &mut State| s.dialog_open = false),
            )
            .action(
                Button::new("Reset")
                    .variant(ButtonVariant::Text)
                    .key("dialog-reset")
                    .ink(&s.ink)
                    .build(t)
                    .on_press(|s: &mut State| {
                        s.dialog_open = false;
                        s.theme = Theme::light();
                        s.message = "Settings reset".into();
                    }),
            )
            .build(t)
            .modal(true, |s: &mut State| s.dialog_open = false);
        layers.push(
            el().key("dialog-layer")
                .absolute(0.0, 0.0)
                .w_full()
                .h_full()
                .center()
                .child(dialog),
        );
    }

    if s.menu_open {
        let item = |label: &str, key: &'static str, selected: bool| {
            let choice = label.to_owned();
            MenuItem::new(label)
                .key(key)
                .selected(selected)
                .ink(&s.ink)
                .build(t)
                .on_press(move |s: &mut State| {
                    s.food = choice.clone();
                    s.menu_open = false;
                })
        };
        let menu = Menu::new()
            .key("menu")
            .width(224.0)
            .child(item("Pizza", "menu-pizza", s.food == "Pizza"))
            .child(item("Pasta", "menu-pasta", s.food == "Pasta"))
            .child(Divider::new().build(t).mt(8.0).mb(8.0))
            .child(item("Salad", "menu-salad", s.food == "Salad"))
            .build(t)
            .modal(true, |s: &mut State| s.menu_open = false)
            .anchored("open-menu", rune::Side::Bottom, rune::Align::Start, 8.0);
        layers.push(menu);
    }

    layers
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = &s.theme;
    let c = &t.color;

    let content = col()
        .key("gallery-scroll")
        .w_full()
        .grow(1.0)
        .min_h(0.0)
        .padding(24.0)
        .gap(24.0)
        .scroll(ScrollSpec {
            stick_bottom: false,
        })
        .child(
            text("rune-material")
                .font(t.type_scale.headline_small)
                .color(c.on_surface),
        )
        .child(
            text("Material 3 tokens and components for Rune")
                .font(t.type_scale.body_medium)
                .color(c.on_surface_variant),
        )
        .child(section(t, "Actions", actions(s)))
        .child(section(t, "Selection", selection(s)))
        .child(section(t, "Fields", fields(s)))
        .child(section(t, "Chips", chips(s)))
        .child(section(t, "Cards", cards(s)))
        .child(section(t, "Badges and progress", indicators(s)))
        .child(section(t, "Tabs", tabs_section(s)))
        .child(section(t, "Lists", lists(s)))
        .child(section(t, "Sliders", slider_section(s)))
        .child(section(t, "Snackbars", snackbar_section(s)))
        .child(section(t, "Overlays", overlay_demo(s)))
        .child(section(t, "Theme", theme_controls(s)))
        .child(el().h(1.0).w_full().fill(c.outline_variant))
        .child(
            text(s.message.clone())
                .font(t.type_scale.label_large)
                .color(c.on_surface_variant),
        );

    col()
        .w_full()
        .h_full()
        .fill(c.surface)
        .child(content)
        .children(overlays(s))
}

fn main() {
    rune::run(AppSpec::<State> {
        scenes: SCENES,
        setup: Some(|s, waker| s.fonts.watch(waker)),
        on_key: Some(|s, event, tx| s.fonts.on_key(event, &mut s.theme, tx)),
        poll: Some(poll),
        ..AppSpec::new("rune-material gallery", (900.0, 1000.0), init, view)
    });
}

fn poll(s: &mut State, now: f64, tx: &mut rune::Tx) -> bool {
    let changed = s.ink.poll(now);
    s.fonts.poll(&mut s.theme, tx) || changed
}
