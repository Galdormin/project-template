//! The pause menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    menus::Menu,
    screens::Screen,
    theme::widget::{Header, UiButton, UiRoot},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        #PauseMenu
        @UiRoot
        GlobalZIndex(2)
        DespawnOnExit::<Menu>(Menu::Pause)
        Children [
            @Header Text("Game paused"),
            @UiButton { @text: "Continue" } on(close_menu),
            @UiButton { @text: "Settings" } on(open_settings_menu),
            @UiButton { @text: "Quit to title" } on(quit_to_title),
        ]
    });
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
